mod auth;
mod cors;
mod db;
mod endpoints;

use config::JSON_BODY_LIMIT;
use core::settings::SETTINGS;
use sqlx::postgres::PgPool;
use std::env;
use std::net::SocketAddr;

pub async fn start() {
    core::settings::init().await;
    let db_pool = db::get_pool(&SETTINGS.get().unwrap()).await;
    _start(db_pool).await;
}

//auto reload on code change
//see: https://github.com/seanmonstar/warp/blob/master/examples/autoreload.rs
#[cfg(feature = "local")]
pub async fn _start(pool: PgPool) {
    // todo: de-duplicate this somehow.
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(cors::get_cors_actix().finish())
            .app_data(actix_web::web::JsonConfig::default().limit(JSON_BODY_LIMIT as usize))
            .configure(endpoints::user::configure)
    });

    // if listenfd doesn't take a TcpListener (i.e. we're not running via
    // the command above), we fall back to explicitly binding to a given
    // host:port.
    let server = if let Some(l) = listenfd::ListenFd::from_env().take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(get_addr()).unwrap()
    };

    server.run().await.unwrap();
}

#[cfg(not(feature = "local"))]
pub async fn _start(pool: PgPool) {
    // todo: de-duplicate this somehow.
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(cors::get_cors_actix().finish())
            .app_data(actix_web::web::JsonConfig::default().limit(JSON_BODY_LIMIT as usize))
            .configure(endpoints::user::configure)
    })
    .bind(get_addr())
    .unwrap()
    .run()
    .await
    .unwrap();
}

fn get_addr() -> SocketAddr {
    let mut port = SETTINGS.get().unwrap().api_port;

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
