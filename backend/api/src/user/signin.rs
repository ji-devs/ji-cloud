use std::convert::Infallible;
use futures_util::future::TryFutureExt;
use warp::{
    http::StatusCode,
    Filter, 
    reject::Reject,
    Rejection
};
use ji_cloud_shared::{
    auth::{SigninSuccess, SingleSignOnSuccess, AuthClaims, JWT_COOKIE_NAME, CSRF_HEADER_NAME},
    user::UserRole,
    api::result::ResultResponse,
    api::endpoints::{
        ApiEndpoint,
        user::SingleSignOn,
    }
};
use crate::reject::{CustomWarpRejection, NoAuth, AuthCreate, InternalError};
use crate::db::Db;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::settings::SETTINGS;
use super::auth::reply_signin_auth;
use crate::reply::HandlerResult;
use jsonwebtoken::{encode, Header, dangerous_unsafe_decode};
use crate::db::{pg_pool, PgPool, get_db};

//the user_id is already validated in terms of firebase auth
//now we need to check with the database
//login handler doesn't use the usual wrapper since it needs to set the header
pub async fn handle_signin_credentials(user_id:String, pool:PgPool) -> Result<impl warp::Reply, warp::Rejection> {

    log::info!("Firebase is valid! user id is: {}", user_id);

    let db = get_db(pool)?;

    let user = super::queries::get_by_id(&db, &user_id).ok_or(NoAuth::rejection())?;

    reply_signin_auth(user_id, user.roles, false)

}


//the claims are already validated from cookie and db lookup 
//no need to validate anything else
pub async fn handle_get_sso_jwt(auth:AuthClaims) -> HandlerResult< <SingleSignOn as ApiEndpoint>::Res, <SingleSignOn as ApiEndpoint>::Err> {

    log::info!("Firebase is valid! user id is: {}", auth.id);

    let claims = AuthClaims {
        id: auth.id,
        csrf: None,
        roles: auth.roles
    };

    let jwt = encode(&Header::default(), &claims, &SETTINGS.get().unwrap().jwt_encoding_key).map_err(|_| InternalError::rejection())?;

    Ok(Ok(SingleSignOnSuccess{jwt}))
}
