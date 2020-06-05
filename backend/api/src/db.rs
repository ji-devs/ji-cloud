use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use serde::{Serialize, Deserialize};
use jsonwebtoken as jwt;
use std::collections::HashMap;
use std::convert::Infallible;
use futures_util::future::TryFutureExt;
use warp::{
    http::StatusCode,
    Filter, 
    reject::Reject,
    Rejection
};
use crate::settings::{SETTINGS, DB_CONNECTION, RemoteTarget};
use crate::reject::{CustomWarpRejection, PgPoolError};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type Db = PooledConnection<ConnectionManager<PgConnection>>;

pub fn pg_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(&*DB_CONNECTION);
    if SETTINGS.db_target == RemoteTarget::Local {
        Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("Postgres connection pool could not be created (local)")
    } else {
        Pool::new(manager).expect("Postgres connection pool could not be created (remote)")
    }

}

pub fn get_db(pool:PgPool) -> Result<Db, warp::Rejection> {
    match pool.get() {
        Ok(conn) => Ok(conn),
        Err(_) => Err(PgPoolError::rejection()),
    }
}
