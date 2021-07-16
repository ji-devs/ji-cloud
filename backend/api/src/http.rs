mod cors;
mod endpoints;

use crate::{
    error::BasicError,
    google, s3,
    service::{mail, ServiceData},
};
use actix_service::Service;
use actix_web::{dev::Server, HttpResponse};
use actix_web::{
    dev::{MessageBody, ServiceRequest, ServiceResponse},
    web::Data,
};
use config::JSON_BODY_LIMIT;
use core::{
    http::{get_addr, get_tcp_fd},
    settings::RuntimeSettings,
};
use futures::Future;
use paperclip::actix::{api_v2_operation, NoContent, OpenApiExt};
use sqlx::postgres::PgPool;
use std::{net::TcpListener, sync::Arc};

fn log_ise<B: MessageBody, T>(
    request: ServiceRequest,
    srv: &mut T,
) -> impl Future<Output = actix_web::Result<T::Response>>
where
    T: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let uri: serde_json::Value = request.uri().to_string().into();
    let method: serde_json::Value = request.method().to_string().into();

    let future = srv.call(request);
    async {
        let mut result = future.await?;
        if result.status() == 500 {
            let response: &mut actix_http::Response<_> = result.response_mut();

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
    gcs: Option<google::storage::Client>,
    algolia: Option<crate::algolia::Client>,
    algolia_key_store: Option<crate::algolia::SearchKeyStore>,
    jwk_verifier: Arc<crate::jwk::JwkVerifier>,
    mail_client: Option<mail::Client>,
) -> anyhow::Result<()> {
    let app = build(
        pool,
        settings,
        s3,
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
    gcs: Option<google::storage::Client>,
    algolia: Option<crate::algolia::Client>,
    algolia_key_store: Option<crate::algolia::SearchKeyStore>,
    jwk_verifier: Arc<crate::jwk::JwkVerifier>,
    mail_client: Option<mail::Client>,
) -> anyhow::Result<Application> {
    let local_insecure = settings.is_local();
    let api_port = settings.api_port;

    let s3 = s3.map(ServiceData::new);
    let gcs = gcs.map(ServiceData::new);
    let algolia = algolia.map(ServiceData::new);
    let algolia_key_store = algolia_key_store.map(ServiceData::new);
    let mail_client = mail_client.map(ServiceData::new);

    let server = actix_web::HttpServer::new(move || {
        let server = actix_web::App::new()
            .data(pool.clone())
            .data(settings.clone());

        let server = match s3.clone() {
            Some(s3) => server.app_data(s3),
            None => server,
        };

        let server = match gcs.clone() {
            Some(gcs) => server.app_data(gcs),
            None => server,
        };

        let server = match algolia.clone() {
            Some(algolia) => server.app_data(algolia),
            None => server,
        };

        let server = match algolia_key_store.clone() {
            Some(algolia_key_store) => server.app_data(algolia_key_store),
            None => server,
        };

        let server = match mail_client.clone() {
            Some(mail_client) => server.app_data(mail_client),
            None => server,
        };

        server
            .app_data(Data::from(jwk_verifier.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap_fn(log_ise)
            .wrap(cors::get(local_insecure))
            .service(get_spec)
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
            .wrap_api()
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
            .route("/", paperclip::actix::web::get().to(no_content_response))
            .with_json_spec_at("/spec.json")
            .build()
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

#[actix_web::get("/spec")]
async fn get_spec() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/spec-explorer.html"))
}

fn default_route() -> HttpResponse {
    HttpResponse::NotFound().json(BasicError::with_message(
        http::StatusCode::NOT_FOUND,
        "Route not found".to_owned(),
    ))
}

#[api_v2_operation]
async fn no_content_response() -> NoContent {
    NoContent
}

pub fn bad_request_handler() -> actix_web::Error {
    BasicError::new(http::StatusCode::BAD_REQUEST).into()
}
