mod auth;
mod cors;
mod endpoints;

use config::JSON_BODY_LIMIT;
use core::settings::Settings;
use sqlx::postgres::PgPool;
use std::env;
use std::net::SocketAddr;

#[cfg(feature = "local")]
fn get_tcp_fd() -> Option<std::net::TcpListener> {
    listenfd::ListenFd::from_env().take_tcp_listener(0).unwrap()
}

#[cfg(not(feature = "local"))]
fn get_tcp_fd() -> Option<std::net::TcpListener> {
    // we don't have listenfd here.
    None
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
            .wrap(cors::get_cors_actix(local_insecure).finish())
            .app_data(actix_web::web::JsonConfig::default().limit(JSON_BODY_LIMIT as usize))
            .configure(endpoints::user::configure)
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

fn get_addr(default: u16) -> SocketAddr {
    let mut port = default;

    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {
                    port = n;
                }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };

    ([0, 0, 0, 0], port).into()
}
