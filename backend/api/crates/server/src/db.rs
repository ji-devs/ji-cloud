use sqlx::postgres::{PgPool, PgPoolOptions, PgConnectOptions};

use core::settings::{DbTarget, DbEndpoint, Settings};
use config::DB_POOL_CONNECTIONS;

pub async fn get_pool(settings:&Settings) -> PgPool {
    //let db_connection_string = &settings.db_credentials.to_string();
    let db_target = settings.db_target;
    let n_connections = if db_target == DbTarget::Local || db_target == DbTarget::Proxy { 1 } else { DB_POOL_CONNECTIONS };

    let credentials = &settings.db_credentials;

    let connect_options = PgConnectOptions::new()
        .username(&credentials.user)
        .password(&credentials.pass)
        .database(&credentials.dbname);

    let connect_options = match &credentials.endpoint {
        DbEndpoint::Tcp(host, port) => {
            connect_options.host(host).port(*port)
        },
        DbEndpoint::Socket(path) => {
            connect_options.socket(path)
        }
    };

    PgPoolOptions::new()
        .max_connections(n_connections)
        .connect_with(connect_options)
        .await
        .expect("Postgres connection pool could not be created (local)")


}
