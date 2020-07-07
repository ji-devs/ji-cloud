use sqlx::postgres::PgPool;
use crate::settings::Settings;
use ji_cloud_shared::backend::settings::{DbTarget, DB_POOL_CONNECTIONS};

pub async fn get_pool(settings:&Settings) -> PgPool {
    let db_connection_string = &settings.db_connection_string;
    let db_target = settings.db_target;
    let n_connections = if db_target == DbTarget::Local || db_target == DbTarget::Proxy { 1 } else { DB_POOL_CONNECTIONS };

    PgPool::builder()
        .max_size(n_connections)
        .build(db_connection_string)
        .await
        .expect("Postgres connection pool could not be created (local)")

}
