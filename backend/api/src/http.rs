mod auth;
mod cors;
mod endpoints;

use actix_service::Service;
use actix_web::dev::{MessageBody, ServiceRequest, ServiceResponse};
use config::JSON_BODY_LIMIT;
use core::{
    http::{get_addr, get_tcp_fd},
    settings::Settings,
};
use futures::Future;
use sqlx::postgres::PgPool;
use std::env;
use std::net::SocketAddr;

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

#[actix_web::main]
pub async fn run(pool: PgPool, settings: Settings) -> anyhow::Result<()> {
    let local_insecure = settings.local_insecure;
    let api_port = settings.api_port;
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(pool.clone())
            .data(settings.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap_fn(log_ise)
            .wrap(cors::get(local_insecure).finish())
            .app_data(actix_web::web::JsonConfig::default().limit(JSON_BODY_LIMIT as usize))
            .configure(endpoints::user::configure)
            .configure(endpoints::category::configure)
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
