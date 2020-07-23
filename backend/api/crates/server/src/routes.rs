#![feature(iterator_fold_self)]

use sqlx::postgres::PgPool;
use warp::{
    Filter,
    path,
};

use shared::{
    api::endpoints::user::{Signin,SingleSignOn,Register,Profile},
};
use config::JSON_BODY_LIMIT;

use crate::reply::ReplyExt;
use crate::auth::{has_auth_cookie_and_db_no_csrf, has_auth_no_db, has_firebase_auth};
use crate::reject::handle_rejection;
use crate::{async_clone_fn, async_clone_cb};
use crate::endpoints::user;
use super::cors::get_cors;


// blocked on blocked on https://github.com/seanmonstar/warp/issues/621
/*
fn path_from_str(uri:&str) -> impl Filter + Clone {
    let mut iter = uri.split('/').map(path::path);

    let first = iter.next().unwrap();

    iter.fold(first, |acc, val| acc.and(val))
}
*/


//All of our routes
pub async fn get_routes(pool:PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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
        .and_then(async_clone_fn!(pool; |user| { user::handle_signin_credentials(user, pool).await }))
    .or(
        //signin to get a new jwt only requires the cookie (use case is single signin)
        path!("user" / "get-sso-jwt")
            .and(has_auth_cookie_and_db_no_csrf(pool.clone()))
            .and_then(|auth| async move { user::handle_get_sso_jwt(auth).await.warp_reply() })
    )
    .or(
        //registration only requires the firebase jwt
        warp::post()
            .and(path!("user" / "register"))
            .and(has_firebase_auth())
            .and(json_body_limit())
            .and_then(async_clone_fn!(pool; |user, form| { user::handle_register(user, form, pool).await }))
    )
}

//Protected routes (requires user is signed in)
pub fn protected_routes(pool:PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("user" / "profile")
        .and(has_auth_no_db())
        .and_then(async_clone_fn!(pool; |auth| { user::handle_get_profile(auth, pool).await.warp_reply() }))
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
