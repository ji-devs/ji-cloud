pub mod category;
pub mod user;

use config::DB_POOL_CONNECTIONS;
use core::settings::{DbEndpoint, Settings};
use futures::FutureExt;
use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    Executor,
};

pub async fn get_pool(settings: &Settings) -> anyhow::Result<PgPool> {
    //let db_connection_string = &settings.db_credentials.to_string();
    let credentials = &settings.db_credentials;

    let connect_options = PgConnectOptions::new()
        .username(&credentials.user)
        .password(&credentials.pass)
        .database(&credentials.dbname);

    let connect_options = match &credentials.endpoint {
        DbEndpoint::Tcp(host, port) => connect_options.host(host).port(*port),
        DbEndpoint::Socket(path) => connect_options.socket(path),
    };

    let pool = PgPoolOptions::new()
        .max_connections(DB_POOL_CONNECTIONS)
        .after_connect(|conn| {
            async move {
                conn.execute(include_str!("category-tree.sql"))
                    .await
                    .map(drop)
            }
            .boxed()
        })
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}
