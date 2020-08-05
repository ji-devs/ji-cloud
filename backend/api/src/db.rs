pub(crate) mod category;
pub(crate) mod user;

use config::DB_POOL_CONNECTIONS;
use futures::FutureExt;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgConnectOptions},
    Executor,
};

pub async fn get_pool(connect_options: PgConnectOptions) -> anyhow::Result<PgPool> {
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
