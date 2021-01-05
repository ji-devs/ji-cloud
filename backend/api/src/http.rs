mod cors;
mod endpoints;

use crate::{error::BasicError, jwkkeys::JwkVerifier, s3};
use actix_service::Service;
use actix_web::dev::{MessageBody, ServiceRequest, ServiceResponse};
use actix_web::HttpResponse;
use config::JSON_BODY_LIMIT;
use core::{
    http::{get_addr, get_tcp_fd},
    settings::RuntimeSettings,
};
use futures::Future;
use paperclip::actix::OpenApiExt;
use sqlx::postgres::PgPool;
use std::sync::Arc;

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

#[actix_web::main]
pub async fn run(
    pool: PgPool,
    settings: RuntimeSettings,
    jwk_verifier: Arc<JwkVerifier>,
    s3: s3::Client,
    algolia: crate::algolia::Client,
) -> anyhow::Result<()> {
    let local_insecure = settings.is_local();
    let api_port = settings.api_port;
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(pool.clone())
            .data(settings.clone())
            .data(s3.clone())
            .data(algolia.clone())
            .app_data(jwk_verifier.clone())
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
            .with_json_spec_at("/spec.json")
            .build()
    });

    // if listenfd doesn't take a TcpListener (i.e. we're not running via
    // the command above), we fall back to explicitly binding to a given
    // host:port.
    let server: _ = if let Some(l) = get_tcp_fd() {
        server.listen(l)?
    } else {
        server.bind(get_addr(api_port))?
    };

    server.run().await.unwrap();

    Ok(())
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

fn bad_request_handler() -> actix_web::Error {
    BasicError::new(http::StatusCode::BAD_REQUEST).into()
}
