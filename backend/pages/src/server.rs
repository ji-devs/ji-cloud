mod cors;
mod routes;

use crate::templates::direct;
use actix_service::Service;
use actix_web::{
    dev::{MessageBody, ServiceRequest, ServiceResponse},
    web,
};
use config::JSON_BODY_LIMIT;
use core::{
    http::{get_addr, get_tcp_fd},
    settings::RuntimeSettings,
};
use futures::Future;

// todo: dedup this with api
fn log_ise<B: MessageBody, T>(
    req: ServiceRequest,
    srv: &mut T,
) -> impl Future<Output = actix_web::Result<T::Response>>
where
    T: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let uri: serde_json::Value = req.uri().to_string().into();
    let method: serde_json::Value = req.method().to_string().into();

    let fut = srv.call(req);
    async {
        let mut res = fut.await?;
        if res.status() == 500 {
            let resp: &mut actix_http::Response<_> = res.response_mut();

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
            .data(settings.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap_fn(log_ise)
            .wrap(cors::get(local_insecure))
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
        server.bind(get_addr(pages_port))?
    };

    server.run().await.unwrap();

    Ok(())
}
