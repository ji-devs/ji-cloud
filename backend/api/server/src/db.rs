use sqlx::postgres::{PgPool, PgConnectOptions};
use crate::settings::Settings;
use ji_cloud_shared::backend::settings::{DbTarget, DbEndpoint, DB_POOL_CONNECTIONS};

pub async fn get_pool(settings:&Settings) -> PgPool {
    //let db_connection_string = &settings.db_credentials.to_string();
    let db_target = settings.db_target;
    let n_connections = if db_target == DbTarget::Local || db_target == DbTarget::Proxy { 1 } else { DB_POOL_CONNECTIONS };

    let credentials = &settings.db_credentials;

    let options = PgConnectOptions::new()
        .username(&credentials.user)
        .password(&credentials.pass)
        .database(&credentials.dbname);

    let options = match &credentials.endpoint {
        DbEndpoint::Tcp(host, port) => {
            options.host(host).port(*port)
        },
        DbEndpoint::Socket(path) => {
            options.socket(path)
        }
    };

    PgPool::builder()
        .max_size(n_connections)
        .build_with(options)
        .await
        .expect("Postgres connection pool could not be created (local)")

}
