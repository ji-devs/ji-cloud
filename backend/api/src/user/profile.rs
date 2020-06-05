use serde::Serialize;
use std::convert::Infallible;
use futures_util::future::TryFutureExt;
use warp::{
    http::StatusCode,
    Filter, 
    reject::Reject,
    Rejection
};
use ji_cloud_shared::{
    auth::AuthClaims,
    user::{User, NoSuchUserError},
};
use crate::db::{pg_pool, PgPool, get_db};
use crate::reply::{reply_err, reply_ok};
use super::queries::{get_by_email, get_by_id};

pub async fn handle_get_profile(claims:AuthClaims, pool:PgPool) -> Result<impl warp::Reply, Rejection> {
    let db = get_db(pool)?;

    match get_by_id(&db, &claims.id) {
        None => reply_err(NoSuchUserError{}),
        Some(user) => reply_ok(user)
    }

}
