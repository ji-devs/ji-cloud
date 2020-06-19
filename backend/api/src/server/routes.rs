use diesel::{self, prelude::*};
use warp::{
    http::Method,
    Filter,
    path
};

use crate::settings::SETTINGS;
use ji_cloud_shared::backend::settings::JSON_BODY_LIMIT;
use crate::user::{self, auth::{has_auth_cookie_and_db_no_csrf, has_auth_no_db, has_auth_full, has_firebase_auth }};
use crate::reject::handle_rejection;
use crate::db::{pg_pool, PgPool};
use crate::{async_clone_fn, async_clone_cb};
use super::cors::get_cors;
use std::net::SocketAddr;

//All of our routes
pub fn get_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let pool = pg_pool();
    auth_routes(pool.clone())
        .or(protected_routes(pool.clone()))
        .or(open_routes(pool.clone()))
        .recover(handle_rejection)
        .with(get_cors())
}

//Auth flows
pub fn auth_routes(pool:PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //main signin only requires the firebase jwt
    warp::post()
        .and(path!("user" / "signin"))
        .and(has_firebase_auth())
        .and_then(async_clone_fn!(pool; |user| { user::signin::handle_signin_credentials(user, pool).await }))
    .or(
        //signin to get a new jwt only requires the cookie (use case is single signin)
        path!("user" / "get-signin-jwt")
            .and(has_auth_cookie_and_db_no_csrf(pool.clone()))
            .and_then(user::signin::handle_get_signin_jwt)
    )
    .or(
        //registration only requires the firebase jwt
        warp::post()
            .and(path!("user" / "register"))
            .and(has_firebase_auth())
            .and(json_body_limit())
            .and_then(async_clone_fn!(pool; |user, form| { user::register::handle_register(user, form, pool).await }))
    )
}

//Protected routes (requires user is signed in)
pub fn protected_routes(pool:PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("user" / "profile")
        .and(has_auth_no_db())
        .and_then(async_clone_fn!(pool; |auth| { user::profile::handle_get_profile(auth, pool).await }))
}

//Open/Public routes
pub fn open_routes(pool:PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    fn handle_index() -> String {
        format!("ready to rock!!!!")
    }


    path::end().map(handle_index)
}

//Decode the body as a specific json type
//and limit the length to prevent DoS
fn json_body_limit<T: serde::de::DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(JSON_BODY_LIMIT).and(warp::body::json())
}
