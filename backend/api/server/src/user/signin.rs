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
    user::{UserRole, NoSuchUserError},
    api::result::ResultResponse,
    api::endpoints::{
        ApiEndpoint,
        user::SingleSignOn,
    }
};
use crate::reject::{CustomWarpRejection, NoAuth, InternalError};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::settings::SETTINGS;
use super::auth::reply_signin_auth;
use crate::reply::HandlerResult;
use jsonwebtoken::{encode, Header, dangerous_unsafe_decode};
use sqlx::postgres::PgPool;

//the user_id is already validated in terms of firebase auth
//now we need to check with the database
//login handler doesn't use the usual wrapper since it needs to set the header
pub async fn handle_signin_credentials(user_id:String, db:PgPool) -> Result<impl warp::Reply, warp::Rejection> {

    log::info!("Firebase is valid! user id is: {}", user_id);


    match super::queries::get_by_id(&db, &user_id) {
        Some(user) => reply_signin_auth(user_id, user.roles, false),
        None => {

            //Since the happy path is a WithHeader reply, need wrap the sad path too
            let reply = warp::reply::json(&ResultResponse::Err::<(), NoSuchUserError>(NoSuchUserError{}));
            //TODO: https://github.com/seanmonstar/warp/issues/587#issuecomment-633961421
            //let reply = warp::reply::WithHeader { header: None, reply };

            //placeholder for now until we can really have empty WithHeader
            let reply = warp::reply::with_header(reply, "foo", "bar");
            Ok(reply)
        }
    }


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
