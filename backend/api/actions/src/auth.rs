use futures_util::future::TryFutureExt;
use ji_cloud_shared::{
    auth::{SigninSuccess, RegisterSuccess, AuthClaims, JWT_COOKIE_NAME, CSRF_HEADER_NAME},
    user::UserRole,
    api::result::ResultResponse
};
use serde::{Serialize, Deserialize};
use jsonwebtoken as jwt;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::user::get_by_id;
use crate::settings::SETTINGS;
use ji_cloud_shared::backend::settings::{MAX_SIGNIN_COOKIE, COOKIE_DOMAIN};
use sqlx::postgres::PgPool;

pub enum Error {
    Jwt(jwt::errors::Error),
    Csrf,
    NoUser,
    Firebase
}

pub fn get_claims(token_string:&str) -> Result<AuthClaims, Error> {

    //see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    let key = jsonwebtoken::DecodingKey::from_secret(SETTINGS.get().unwrap().jwt_decoding_key.as_ref());

    let validation = jwt::Validation {validate_exp: false, ..Default::default()};

    jsonwebtoken::decode::<AuthClaims>(&token_string, &key, &validation)
        .map(|decoded| decoded.claims)
        .map_err(|err| Error::Jwt(err))
}

pub async fn check_full(db:&PgPool, token_string:&str, csrf:&str) -> Result<AuthClaims, Error> {
    let claims = check_no_db(token_string, csrf)?;

    if get_by_id(&db, &claims.id).await.is_none() {
        Err(Error::NoUser)
    } else {
        Ok(claims)
    }
}

pub fn check_no_db(token_string:&str, csrf:&str) -> Result<AuthClaims, Error> {
    get_claims(token_string)
        .and_then(|claims| {
            if claims.csrf.as_deref() == Some(csrf) {
                Ok(claims)
            } else {
                Err(Error::Csrf)
            }
        })
}
pub async fn check_no_csrf(db:&PgPool, token_string:&str) -> Result<AuthClaims, Error> {

    let claims = get_claims(token_string)?;
            

    if get_by_id(&db, &claims.id).await.is_none() {
        Err(Error::NoUser)
    } else {
        Ok(claims)
    }
}

pub async fn get_firebase_id(token:&str) -> Result<String, Error> {
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
            .get(&format!("{}/validate-firebase-token/{}", SETTINGS.get().unwrap().remote_target.js_api(), token))
            .header("INTER_SERVER_SECRET", &SETTINGS.get().unwrap().inter_server_secret)
            .send()
            .and_then(|res| res.json::<JsApiResponse>())
            .await
            .map_err(|err| {
                log::warn!("js/firebase error, shouldn't happen: {:?}", err);
                Error::Firebase
            })?;

    if response.valid {

        #[derive(Debug, Serialize, Deserialize)]
        struct FirebaseClaims {
            sub: String,
        }
        let claims:FirebaseClaims = 
            jwt::dangerous_unsafe_decode::<FirebaseClaims>(&token)
                .map_err(|err| Error::Firebase)?
                .claims;

        let user_id = claims.sub;

        Ok(user_id)
    } else {
        Err(Error::Firebase)
    }
}
