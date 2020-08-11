pub(crate) mod category;
pub(crate) mod user;
pub(crate) mod image;

use config::DB_POOL_CONNECTIONS;
use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    Executor,
};

pub async fn get_pool(connect_options: PgConnectOptions) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(DB_POOL_CONNECTIONS)
        .connect_with(connect_options)
        .await?;

    pool.execute(include_str!("view/category-tree.sql")).await?;

    Ok(pool)
}
