use crate::jwt::{check_no_csrf, check_no_db};
use actix_web::{
    cookie::{Cookie, CookieBuilder, SameSite},
    http::{header, HeaderMap, HeaderValue},
    web::Data,
    FromRequest, HttpMessage, HttpResponse,
};
use config::{COOKIE_DOMAIN, MAX_SIGNIN_COOKIE_DURATION};
use core::settings::SETTINGS;
use firebase::get_firebase_id;
use futures::future::FutureExt;
use futures_util::future::BoxFuture;
use jsonwebtoken as jwt;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use shared::auth::{AuthClaims, CSRF_HEADER_NAME, JWT_COOKIE_NAME};
use sqlx::postgres::PgPool;

mod firebase;

pub struct FirebaseUser {
    pub id: FirebaseId,
}

pub struct FirebaseId(pub String);

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let header: &HeaderValue = headers.get(header::AUTHORIZATION)?;

    let header: &str = header.to_str().ok()?;

    // ["Bearer " .. value]
    header.split("Bearer ").nth(1)
}

pub struct AuthError;

impl From<AuthError> for actix_web::Error {
    fn from(_other: AuthError) -> Self {
        HttpResponse::Unauthorized().into()
    }
}
pub enum StatusError {
    Auth,
    InternalServerError,
}

impl From<StatusError> for actix_web::Error {
    fn from(other: StatusError) -> Self {
        match other {
            StatusError::Auth => HttpResponse::Unauthorized().into(),
            StatusError::InternalServerError => HttpResponse::InternalServerError().into(),
        }
    }
}

impl FromRequest for FirebaseUser {
    type Error = StatusError;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        // this whole dance is to avoid cloning the headers.
        let token = match bearer_token(req.headers()) {
            Some(token) => token.to_owned(),
            None => return futures::future::err(StatusError::Auth).boxed(),
        };

        async move {
            get_firebase_id(&token)
                .await
                .map_err(|_| StatusError::InternalServerError)?
                .map(FirebaseId)
                .map(|id| Self { id })
                .ok_or_else(|| StatusError::Auth)
        }
        .boxed()
    }
}

fn csrf_header(headers: &HeaderMap) -> Option<&str> {
    headers.get(CSRF_HEADER_NAME)?.to_str().ok()
}

#[repr(transparent)]
pub struct WrapAuthClaimsNoDb(pub AuthClaims);

impl FromRequest for WrapAuthClaimsNoDb {
    type Error = AuthError;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let cookie = req.cookie(JWT_COOKIE_NAME);
        let csrf = csrf_header(req.headers());

        let (cookie, csrf) = match (cookie, csrf) {
            (Some(cookie), Some(csrf)) => (cookie, csrf),
            _ => return futures::future::err(AuthError),
        };

        futures::future::ready(
            check_no_db(cookie.value(), csrf)
                .map(Self)
                .map_err(|_| AuthError),
        )
    }
}

#[repr(transparent)]
pub struct WrapAuthClaimsCookieDbNoCsrf(pub AuthClaims);

impl FromRequest for WrapAuthClaimsCookieDbNoCsrf {
    type Error = StatusError;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let db: &Data<PgPool> = req.app_data().unwrap();
        let db = db.as_ref().clone();

        let cookie = match req.cookie(JWT_COOKIE_NAME) {
            Some(token) => token.to_owned(),
            None => return futures::future::err(StatusError::Auth).boxed(),
        };

        async move {
            check_no_csrf(&db, &cookie.value())
                .await
                .map_err(|_| StatusError::InternalServerError)?
                .map(Self)
                .ok_or(StatusError::Auth)
        }
        .boxed()
    }
}

pub fn reply_signin_auth(firebase_id: FirebaseId) -> anyhow::Result<(String, Cookie<'static>)> {
    let csrf: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

    // todo: Move FirebaseId to shared and add it to AuthClaims.
    let claims = AuthClaims {
        id: firebase_id.0,
        csrf: Some(csrf.clone()),
    };

    let jwt = jwt::encode(
        &jwt::Header::default(),
        &claims,
        &SETTINGS.get().unwrap().jwt_encoding_key,
    )?;

    let mut cookie = CookieBuilder::new(JWT_COOKIE_NAME, jwt)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(MAX_SIGNIN_COOKIE_DURATION);

    if !SETTINGS.get().unwrap().local_insecure {
        cookie = cookie.domain(COOKIE_DOMAIN);
    }

    Ok((csrf, cookie.finish()))
}
