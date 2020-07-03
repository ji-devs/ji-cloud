use sqlx::postgres::PgPool;
use crate::settings::SETTINGS;
use ji_cloud_shared::backend::settings::{DbTarget, DB_POOL_CONNECTIONS};

/*
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type Db = PooledConnection<ConnectionManager<PgConnection>>;
*/

pub async fn pg_pool() -> PgPool {
    let db_connection_string = &SETTINGS.get().expect("NO SETTINGS SET!").db_connection_string;
    let db_target = SETTINGS.get().unwrap().db_target;
    let n_connections = if db_target == DbTarget::Local || db_target == DbTarget::Proxy { 1 } else { DB_POOL_CONNECTIONS };
    PgPool::builder()
        .max_size(n_connections)
        .build(db_connection_string)
        .await
        .expect("Postgres connection pool could not be created (local)")

}
