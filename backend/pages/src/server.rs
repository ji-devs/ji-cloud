mod cors;
mod routes;

use crate::templates::direct;
use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse},
    http,
    web::{self, Data},
    HttpResponse,
};
use core::{
    config::JSON_BODY_LIMIT,
    http::{get_addr, get_tcp_fd},
    settings::RuntimeSettings,
};
use futures::future::{self, Either};
use regex::Regex;
use sentry::types::protocol::v7::value::Value as JsonValue;

// todo: dedup this with api
fn log_ise<B: MessageBody, T>(
    req: ServiceRequest,
    srv: &T,
) -> impl std::future::Future<Output = actix_web::Result<T::Response>>
where
    T: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let uri: JsonValue = req.uri().to_string().into();
    let method: JsonValue = req.method().to_string().into();

    let fut = srv.call(req);
    async {
        let mut res = fut.await?;
        if res.status() == 500 {
            let resp: &mut actix_web::HttpResponse<B> = res.response_mut();

            if let Some(err) = resp.extensions_mut().remove::<anyhow::Error>() {
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

        Ok(res)
    }
}

pub async fn run(settings: RuntimeSettings) -> anyhow::Result<()> {
    let local_insecure = settings.is_local();
    let pages_port = settings.pages_port;
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Data::new(settings.clone()))
            .wrap_fn(|req, srv| {
                // Check whether the request originates from www.<host>. If it does, return a redirect response with the
                // correct URL in the Location header.
                // Note from Ty: I don't like doing this, but in the absense of a load balancer, and GCP cloud runs
                // inflexible hosts management, this is necessary.
                let request_host = req.connection_info().host().to_owned();

                let regex = Regex::new(r"^(?:www\.)+(?P<host>.*)$").unwrap();
                if let Some(captures) = regex.captures(&request_host) {
                    if let Some(host_capture) = captures.name("host").map(|host| host.as_str()) {
                        let uri = req.uri();
                        let request_scheme = req.connection_info().scheme().to_owned();

                        let url = format!("{request_scheme}://{host_capture}{uri}");

                        return Either::Right(future::ready(Ok(req.into_response(
                            HttpResponse::MovedPermanently()
                                .append_header((http::header::LOCATION, url))
                                .finish(),
                        ))));
                    }
                }

                Either::Left(srv.call(req))
            })
            .wrap(cors::get(local_insecure))
            .wrap(actix_web::middleware::Logger::default())
            .wrap_fn(log_ise)
            .app_data(actix_web::web::JsonConfig::default().limit(JSON_BODY_LIMIT as usize))
            .configure(routes::configure)
            .service(actix_files::Files::new("/static", "./public"))
            .default_service(web::route().to(direct::direct_template_404))
    });

    // if listenfd doesn't take a TcpListener (i.e. we're not running via
    // the command above), we fall back to explicitly binding to a given
    // host:port.
    let server: _ = if let Some(l) = get_tcp_fd() {
        server.listen(l)?
    } else {
        server.bind(get_addr(Some(pages_port)))?
    };

    server.run().await.unwrap();

    Ok(())
}
