use futures_util::future::TryFutureExt;
use warp::{
    Filter, 
    Rejection
};
use ji_cloud_shared::{
    auth::{SigninSuccess, RegisterSuccess, AuthClaims, JWT_COOKIE_NAME, CSRF_HEADER_NAME},
    user::UserRole,
    api::result::ResultResponse
};
use serde::{Serialize, Deserialize};
use jsonwebtoken as jwt;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use actions::user::get_by_id;
use crate::reject::{CustomWarpRejection, NoAuth, PgPoolError, InternalError};
use crate::settings::SETTINGS;
use ji_cloud_shared::backend::settings::{MAX_SIGNIN_COOKIE, COOKIE_DOMAIN};
use sqlx::postgres::PgPool;
use crate::{async_clone_fn, async_clone_cb};
use actions::auth::{get_claims, check_full, check_no_db, check_no_csrf, get_firebase_id};

//This can be used to early exit if there's no bearer token
//is just used internally, the top-level uses specific auth guards (firebase, user, etc.)
fn has_bearer_token() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization")
        .or_else(|_| async {Err(NoAuth::rejection())})
        .map(|bearer_token: String| { bearer_token.replace("Bearer ", "") })
}

//Had some type sig trouble trying to keep this DRY... if you can improve it, great!

pub fn has_auth_full(db:PgPool) -> impl Filter<Extract = (AuthClaims,), Error = Rejection> + Clone {
    warp::filters::cookie::cookie(JWT_COOKIE_NAME)
        .and(
            warp::header::<String>(CSRF_HEADER_NAME)
            .or_else(|_| async {Err(NoAuth::rejection())})
        )
        .and_then(move |cookie:String, csrf:String| {
                let db = db.clone();
                async move {
                    check_full(&db, &cookie, &csrf).await.map_err(|_| NoAuth::rejection())
                }
            }
        )
}

pub fn has_auth_no_db() -> impl Filter<Extract = (AuthClaims,), Error = Rejection> + Clone {
    warp::filters::cookie::cookie(JWT_COOKIE_NAME)
        .and(
            warp::header::<String>(CSRF_HEADER_NAME)
            .or_else(|_| {
                async {Err(NoAuth::rejection())}
            })
        )
        .and_then(|cookie:String, csrf:String| async move {
            check_no_db(&cookie, &csrf).map_err(|_| NoAuth::rejection())
        })
}


pub fn has_auth_cookie_and_db_no_csrf(db:PgPool) -> impl Filter<Extract = (AuthClaims,), Error = Rejection> + Clone {
    warp::filters::cookie::cookie(JWT_COOKIE_NAME)
        .and_then(move |cookie:String| {
                let db = db.clone();
                async move {
                    check_no_csrf(&db, &cookie).await.map_err(|_| NoAuth::rejection())
                }
            }
        )
}

pub fn has_auth_cookie_no_db_nor_csrf() -> impl Filter<Extract = (AuthClaims,), Error = Rejection> + Clone {
    warp::filters::cookie::cookie(JWT_COOKIE_NAME)
        .and_then(|cookie:String| async move {
            get_claims(&cookie).map_err(|_| NoAuth::rejection())
        })
}

//returns the user id if firebase is authenticated
pub fn has_firebase_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    has_bearer_token()
        .and_then(|token: String| async move {
            let token = token.replace("Bearer ", "");
            get_firebase_id(&token).await.map_err(|_| NoAuth::rejection())
        })
}

pub fn reply_signin_auth(user_id:String, roles: Vec<UserRole>, is_register:bool) -> Result<warp::reply::WithHeader<warp::reply::Json>, warp::Rejection> {
    let csrf:String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .collect();

    let claims = AuthClaims {
        id: user_id,
        csrf: Some(csrf.clone()),
        roles
    };

    let jwt = jwt::encode(&jwt::Header::default(), &claims, &SETTINGS.get().unwrap().jwt_encoding_key).map_err(|_| InternalError::rejection())?;

    let reply = {
        if is_register {
            warp::reply::json(&ResultResponse::Ok::<RegisterSuccess, ()>(RegisterSuccess::Signin(csrf)))
        } else {
            warp::reply::json(&ResultResponse::Ok::<SigninSuccess, ()>(SigninSuccess{csrf}))
        }
    };

    let reply = {
        if(SETTINGS.get().unwrap().local_insecure) {
            warp::reply::with_header(reply, "Set-Cookie", &format!("{}={}; HttpOnly; SameSite=Lax; Max-Age={}", JWT_COOKIE_NAME, jwt, MAX_SIGNIN_COOKIE))
        } else {
            warp::reply::with_header(reply, "Set-Cookie", &format!("{}={}; Secure; HttpOnly; SameSite=Lax; Max-Age={}; domain={}", JWT_COOKIE_NAME, jwt, MAX_SIGNIN_COOKIE, COOKIE_DOMAIN))
        }
    };

    Ok(reply)
}
