use std::convert::Infallible;
use futures_util::future::TryFutureExt;
use warp::{
    http::StatusCode,
    Filter, 
    reject::Reject,
    Rejection
};
use ji_cloud_shared::{
    auth::{SigninSuccess, SigninEphemeralSuccess, AuthClaims, JWT_COOKIE_NAME, CSRF_HEADER_NAME},
    user::UserRole,
    response::ResultResponse
};
use crate::reject::{CustomWarpRejection, NoAuth, AuthCreate, InternalError};
use crate::reply::{reply_ok, reply_err};
use crate::db::Db;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::settings::{SETTINGS, JWT_ENCODING_KEY};
use super::auth::reply_signin_auth;
use jsonwebtoken::{encode, Header, dangerous_unsafe_decode};
use crate::db::{pg_pool, PgPool, get_db};
//the user_id is already validated in terms of firebase auth
//now we need to check with the database
pub async fn handle_signin_credentials(user_id:String, pool:PgPool) -> Result<impl warp::Reply, warp::Rejection> {

    log::info!("Firebase is valid! user id is: {}", user_id);

    let db = get_db(pool)?;

    let user = super::queries::get_by_id(&db, &user_id).ok_or(NoAuth::rejection())?;

    reply_signin_auth(user_id, user.roles, false)

}


//the claims are already validated from cookie and db lookup 
//no need to validate anything else
pub async fn handle_get_signin_jwt(auth:AuthClaims) -> Result<impl warp::Reply, warp::Rejection> {

    log::info!("Firebase is valid! user id is: {}", auth.id);

    let claims = AuthClaims {
        id: auth.id,
        csrf: None,
        roles: auth.roles
    };

    let jwt = encode(&Header::default(), &claims, &*JWT_ENCODING_KEY).map_err(|_| InternalError::rejection())?;

    reply_ok(SigninEphemeralSuccess{jwt})
}
