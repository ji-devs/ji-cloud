use std::convert::Infallible;
use futures_util::future::TryFutureExt;
use warp::{
    http::StatusCode,
    Filter, 
    reject::Reject,
    Rejection
};
use ji_cloud_shared::{
    auth::{SigninSuccess, RegisterSuccess, SigninEphemeralSuccess, AuthClaims, JWT_COOKIE_NAME, CSRF_HEADER_NAME},
    user::UserRole,
    response::ResultResponse
};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, dangerous_unsafe_decode, Validation};
use std::collections::HashMap;
use super::queries::{get_by_email, get_by_id};
use crate::reject::{CustomWarpRejection, NoAuth, PgPoolError, InternalError};
use crate::settings::{SETTINGS, MAX_SIGNIN_COOKIE};
use crate::db::{pg_pool, PgPool, get_db};
use crate::{async_clone_fn, async_clone_cb};

//This can be used to early exit if there's no bearer token
//is just used internally, the top-level uses specific auth guards (firebase, user, etc.)
fn has_bearer_token() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization")
        .or_else(|_| async {Err(NoAuth::rejection())})
        .map(|bearer_token: String| { bearer_token.replace("Bearer ", "") })
}


pub fn has_auth(pool:Option<PgPool>) -> impl Filter<Extract = (AuthClaims,), Error = Rejection> + Clone {
    warp::filters::cookie::cookie(JWT_COOKIE_NAME)
        .and_then(async_clone_fn!(pool; |cookie| {
            let claims = get_claims(cookie)?;
            
            if let Some(pool) = pool {
                let db = get_db(pool)?;

                if get_by_id(&db, &claims.id).is_none() {
                    Err(NoAuth::rejection())
                } else {
                    Ok(claims)
                }
            } else {
                Ok(claims)
            }
        }))
}


fn get_claims(token_string:String) -> Result<AuthClaims, Rejection> {

    //see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    let key = jsonwebtoken::DecodingKey::from_secret(SETTINGS.get().unwrap().jwt_decoding_key.as_ref());

    let validation = Validation {validate_exp: false, ..Default::default()};

    jsonwebtoken::decode::<AuthClaims>(&token_string, &key, &validation)
        .map_err(|err| {
            log::warn!("couldn't decode jwt: {:?}", err);
            NoAuth::rejection()
        })
        .map(|decoded| decoded.claims)
}

//returns the user id if firebase is authenticated
pub fn has_firebase_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    has_bearer_token()
        .and_then(|token: String| async move {
            let token = token.replace("Bearer ", "");

            #[derive(Deserialize)]
            struct JsApiResponse {
                valid: bool
            }

            //use the js server to handle this, since it has the official firebase admin sdk
            //it could be done natively in Rust, but depends on:
            //1. https://github.com/Keats/jsonwebtoken/issues/127
            //2. all the specific claim checks (e.g. timestamp comparisons)

            let response:JsApiResponse = 
                reqwest::Client::new()
                    .get(&format!("{}/validate-firebase-token/{}", SETTINGS.get().unwrap().js_api(), token))
                    .header("SHARED_SERVER_SECRET", &SETTINGS.get().unwrap().inter_server_secret)
                    .send()
                    .and_then(|res| res.json::<JsApiResponse>())
                    .await
                    .map_err(|err| NoAuth::rejection())?;

            if response.valid {

                #[derive(Debug, Serialize, Deserialize)]
                struct FirebaseClaims {
                    sub: String,
                }
                let claims:FirebaseClaims = 
                    dangerous_unsafe_decode::<FirebaseClaims>(&token)
                        .map_err(|err| NoAuth::rejection())?
                        .claims;

                let user_id = claims.sub;

                Ok(user_id)
            } else {
                Err(NoAuth::rejection())
            }

        })
}
