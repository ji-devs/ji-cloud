use std::{net::TcpListener, sync::Arc};

use actix_service::Service;
use actix_web::{dev::Server, HttpResponse};
use actix_web::{
    dev::{MessageBody, ServiceRequest, ServiceResponse},
    web::Data,
};
use core::{
    config::JSON_BODY_LIMIT,
    http::{get_addr, get_tcp_fd},
    settings::RuntimeSettings,
};
use futures::Future;
use sqlx::postgres::PgPool;

use crate::{
    error::BasicError,
    service::{self, mail, s3, ServiceData},
};

mod cors;
mod endpoints;

fn log_ise<B: MessageBody, T>(
    request: ServiceRequest,
    srv: &T,
) -> impl Future<Output = actix_web::Result<T::Response>>
where
    T: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let uri: serde_json::Value = request.uri().to_string().into();
    let method: serde_json::Value = request.method().to_string().into();

    let future = srv.call(request);
    async {
        let mut result = future.await?;
        if result.status() == 500 {
            let response: &mut actix_web::HttpResponse<_> = result.response_mut();

            if let Some(err) = response.extensions_mut().remove::<anyhow::Error>() {
                log::error!("ISE while responding to request: {:?}", err);
                sentry::add_breadcrumb(sentry::Breadcrumb {
                    ty: "http".to_owned(),
                    category: Some("request".into()),
                    data: {
                        let mut map = sentry::protocol::Map::new();
                        map.insert("url".to_owned(), uri);
                        map.insert("method".to_owned(), method);
                        map
                    },
                    ..Default::default()
                });

                sentry::integrations::anyhow::capture_anyhow(&err);
            }
        }

        Ok(result)
    }
}

pub struct Application {
    port: u16,
    server: Option<Server>,
}

impl Application {
    pub fn new(port: u16, server: Server) -> Self {
        Self {
            port,
            server: Some(server),
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(mut self) -> Result<(), std::io::Error> {
        if let Some(server) = self.server.take() {
            server.await?;
        }

        Ok(())
    }

    pub async fn stop(mut self, graceful: bool) {
        if let Some(server) = self.server.take() {
            server.stop(graceful).await
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        if let Some(server) = self.server.take() {
            let _ = tokio::spawn(server.stop(false));
        }
    }
}

#[actix_web::main]
pub async fn build_and_run(
    pool: PgPool,
    settings: RuntimeSettings,
    s3: Option<s3::Client>,
    gcp_key_store: Option<service::GcpAccessKeyStore>,
    gcs: Option<service::storage::Client>,
    algolia: Option<crate::algolia::Client>,
    algolia_key_store: Option<crate::algolia::SearchKeyStore>,
    jwk_verifier: Arc<crate::jwk::JwkVerifier>,
    mail_client: Option<mail::Client>,
) -> anyhow::Result<()> {
    let app = build(
        pool,
        settings,
        s3,
        gcp_key_store,
        gcs,
        algolia,
        algolia_key_store,
        jwk_verifier,
        mail_client,
    )?;
    app.run_until_stopped().await?;

    Ok(())
}

pub fn build(
    pool: PgPool,
    settings: RuntimeSettings,
    s3: Option<s3::Client>,
    gcp_key_store: Option<service::GcpAccessKeyStore>,
    gcs: Option<service::storage::Client>,
    algolia: Option<crate::algolia::Client>,
    algolia_key_store: Option<crate::algolia::SearchKeyStore>,
    jwk_verifier: Arc<crate::jwk::JwkVerifier>,
    mail_client: Option<mail::Client>,
) -> anyhow::Result<Application> {
    let local_insecure = settings.is_local();
    let api_port = settings.api_port;

    let s3 = s3.map(ServiceData::new);
    let gcp_key_store = gcp_key_store.map(ServiceData::new);
    let gcs = gcs.map(ServiceData::new);
    let algolia = algolia.map(ServiceData::new);
    let algolia_key_store = algolia_key_store.map(ServiceData::new);
    let mail_client = mail_client.map(ServiceData::new);

    let server = actix_web::HttpServer::new(move || {
        let app = actix_web::App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(settings.clone()));

        let app = match s3.clone() {
            Some(s3) => app.app_data(s3),
            None => app,
        };

        let app = match gcp_key_store.clone() {
            Some(gcp_key_store) => app.app_data(gcp_key_store),
            None => app,
        };

        let app = match gcs.clone() {
            Some(gcs) => app.app_data(gcs),
            None => app,
        };

        let app = match algolia.clone() {
            Some(algolia) => app.app_data(algolia),
            None => app,
        };

        let app = match algolia_key_store.clone() {
            Some(algolia_key_store) => app.app_data(algolia_key_store),
            None => app,
        };

        let app = match mail_client.clone() {
            Some(mail_client) => app.app_data(mail_client),
            None => app,
        };

        app.app_data(Data::from(jwk_verifier.clone()))
            .wrap(cors::get(local_insecure))
            .wrap(actix_web::middleware::Logger::default())
            .wrap_fn(log_ise)
            .app_data(
                actix_web::web::JsonConfig::default()
                    .limit(JSON_BODY_LIMIT as usize)
                    .error_handler(|_, _| bad_request_handler()),
            )
            .app_data(
                actix_web::web::QueryConfig::default().error_handler(|_, _| bad_request_handler()),
            )
            .app_data(
                actix_web::web::PathConfig::default().error_handler(|_, _| bad_request_handler()),
            )
            .external_resource(
                "google_cloud_oauth",
                "https://accounts.google.com/o/oauth2/v2/auth",
            )
            .default_service(actix_web::web::to(default_route))
            .configure(endpoints::user::configure)
            .configure(endpoints::category::configure)
            .configure(endpoints::image::configure)
            .configure(endpoints::audio::configure)
            .configure(endpoints::meta::configure)
            .configure(endpoints::jig::configure)
            .configure(endpoints::module::configure)
            .configure(endpoints::admin::configure)
            .configure(endpoints::animation::configure)
            .configure(endpoints::search::configure)
            .configure(endpoints::media::configure)
            .configure(endpoints::session::configure)
            .configure(endpoints::locale::configure)
            .configure(endpoints::additional_resource::configure)
            .route("/", actix_web::web::get().to(no_content_response))
    });

    // if listenfd doesn't take a TcpListener (i.e. we're not running via
    // the command above), we fall back to explicitly binding to a given
    // host:port.
    let listener = if let Some(l) = get_tcp_fd() {
        l
    } else {
        TcpListener::bind(get_addr(Some(api_port)))?
    };

    let port = listener.local_addr().unwrap().port();

    let server = server.listen(listener)?;

    Ok(Application::new(port, server.run()))
}

fn default_route() -> HttpResponse {
    HttpResponse::NotFound().json(BasicError::with_message(
        http::StatusCode::NOT_FOUND,
        "Route not found".to_owned(),
    ))
}

async fn no_content_response() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

pub fn bad_request_handler() -> actix_web::Error {
    BasicError::new(http::StatusCode::BAD_REQUEST).into()
}
