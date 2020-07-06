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
    api::endpoints::{
        ApiEndpoint,
        user::Profile,
    }
};
use crate::reply::HandlerResult;
use super::queries::{get_by_email, get_by_id};

use sqlx::postgres::PgPool;

pub async fn handle_get_profile(claims:AuthClaims, db:PgPool) -> HandlerResult< <Profile as ApiEndpoint>::Res, <Profile as ApiEndpoint>::Err> {

    Ok(match get_by_id(&db, &claims.id) {
        None => Err(NoSuchUserError{}),
        Some(user) => Ok(user)
    })
}

