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

fn log_ise<B: MessageBody, T>(
    req: ServiceRequest,
    srv: &mut T,
) -> impl Future<Output = actix_web::Result<T::Response>>
where
    T: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let fut = srv.call(req);
    async {
        let res = fut.await?;

        if res.status() == 500 {
            if let Some(err) = res.response().extensions().get::<anyhow::Error>() {
                log::error!("ISE while responding to request: {:?}", err);
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
            .wrap(cors::get(local_insecure).finish())
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
