use crate::{
    error::BasicError,
    jwkkeys::{self, JwkVerifier},
    jwt::{check_no_csrf, check_no_db},
    more_futures::ReadyOrNot,
};
use actix_web::{
    cookie::{Cookie, CookieBuilder, SameSite},
    http::{header, HeaderMap, HeaderValue},
    web::Data,
    FromRequest, HttpMessage,
};
use config::{COOKIE_DOMAIN, MAX_SIGNIN_COOKIE_DURATION};
use core::settings::RuntimeSettings;
use futures::future::{self, FutureExt};
use http::StatusCode;
use jsonwebtoken as jwt;
use jwt::EncodingKey;
use paperclip::actix::{Apiv2Schema, Apiv2Security};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use shared::domain::{
    auth::{AuthClaims, CSRF_HEADER_NAME, JWT_COOKIE_NAME},
    user::UserScope,
};
use sqlx::postgres::PgPool;
use std::{marker::PhantomData, sync::Arc};
use uuid::Uuid;

fn try_insecure_decode(token: &str) -> Option<FirebaseId> {
    let claims: jwkkeys::Claims = jsonwebtoken::dangerous_insecure_decode(token).ok()?.claims;
    let user_id = claims.sub;
    Some(FirebaseId(user_id))
}

#[derive(Apiv2Security)]
#[openapi(
    apiKey,
    alias = "firebaseApiKey",
    in = "header",
    name = "Authorization",
    description = "Use format 'Bearer TOKEN'"
)]
pub struct FirebaseUser {
    pub id: FirebaseId,
}

pub struct FirebaseId(pub String);

// stolen from the stdlib and modified (to work on stable)
fn split_once(s: &'_ str, delimiter: char) -> Option<(&'_ str, &'_ str)> {
    let start = s.find(delimiter)?;
    let end = start + delimiter.len_utf8();
    Some((&s[..start], &s[end..]))
}

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let header: &HeaderValue = headers.get(header::AUTHORIZATION)?;

    split_once(header.to_str().ok()?, ' ')
        .filter(|(kind, _)| kind.eq_ignore_ascii_case("bearer"))
        .map(|(_, token)| token)
}

impl FromRequest for FirebaseUser {
    type Error = BasicError;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let settings: &Data<RuntimeSettings> = req.app_data().unwrap();
        let settings = settings.clone();
        let jwk_verifier: &Arc<JwkVerifier> = req.app_data().unwrap();
        let jwk_verifier = jwk_verifier.clone();

        // this whole dance is to avoid cloning the headers.
        let token = match bearer_token(req.headers()) {
            Some(token) => token.to_owned(),
            None => {
                return futures::future::err(BasicError::with_message(
                    StatusCode::UNAUTHORIZED,
                    "Unauthorized: Missing Bearer Token".to_owned(),
                ))
                .into()
            }
        };

        let invalid_token = || {
            BasicError::with_message(
                StatusCode::UNAUTHORIZED,
                "Unauthorized: Invalid Token".to_owned(),
            )
        };

        // HACK for testing.
        if settings.firebase_assume_valid() {
            return futures::future::ready(
                try_insecure_decode(&token)
                    .map(|id| Self { id })
                    .ok_or_else(invalid_token),
            )
            .into();
        }

        async move {
            // todo: more specific errors.
            let id = jwk_verifier
                .verify(&token, 3)
                .await
                .map_err(|_| invalid_token())?;

            Ok(Self { id })
        }
        .boxed()
        .into()
    }
}

fn csrf_header(headers: &HeaderMap) -> Option<&str> {
    headers.get(CSRF_HEADER_NAME)?.to_str().ok()
}

fn check_cookie_csrf<'a>(
    cookie: Option<Cookie<'a>>,
    csrf: Option<&'a str>,
) -> Result<(Cookie<'a>, &'a str), BasicError> {
    match (cookie, csrf) {
        (Some(cookie), Some(csrf)) => Ok((cookie, csrf)),

        (None, Some(_)) => Err(BasicError::with_message(
            StatusCode::UNAUTHORIZED,
            "Unauthorized: missing cookie".to_owned(),
        )),

        (Some(_), None) => Err(BasicError::with_message(
            StatusCode::UNAUTHORIZED,
            "Unauthorized: missing X-CSRF header".to_owned(),
        )),

        (None, None) => Err(BasicError::with_message(
            StatusCode::UNAUTHORIZED,
            "Unauthorized: missing X-CSRF header, missing cookie".to_owned(),
        )),
    }
}

#[derive(Apiv2Security)]
#[openapi(
    apiKey,
    in = "header",
    name = "Authorization",
    description = "Use format 'Bearer TOKEN'"
)]
#[repr(transparent)]
pub struct WrapAuthClaimsNoDb(pub AuthClaims);

impl FromRequest for WrapAuthClaimsNoDb {
    type Error = BasicError;
    type Future = future::Ready<Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let cookie = req.cookie(JWT_COOKIE_NAME);
        let csrf = csrf_header(req.headers());
        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");

        let (cookie, csrf) = match check_cookie_csrf(cookie, csrf) {
            Ok((cookie, csrf)) => (cookie, csrf),
            Err(e) => return futures::future::err(e),
        };

        future::ready(
            check_no_db(cookie.value(), csrf, &settings.jwt_decoding_key())
                .map_err(|_| {
                    BasicError::with_message(
                        StatusCode::UNAUTHORIZED,
                        "Unauthorized: bad JWT".to_owned(),
                    )
                })
                .and_then(|it| {
                    it.ok_or_else(|| {
                        BasicError::with_message(
                            StatusCode::UNAUTHORIZED,
                            "Unauthorized: CSRF mismatch".to_owned(),
                        )
                    })
                })
                .map(Self),
        )
    }
}

// fixme: replace with const-generics once stable
pub trait Scope {
    fn scope() -> UserScope;
}

#[derive(Apiv2Schema)]
pub struct ScopeManageCategory;

impl Scope for ScopeManageCategory {
    fn scope() -> UserScope {
        UserScope::ManageCategory
    }
}

#[derive(Apiv2Schema)]
pub struct ScopeManageImage;

impl Scope for ScopeManageImage {
    fn scope() -> UserScope {
        UserScope::ManageImage
    }
}

#[derive(Apiv2Schema)]
pub struct ScopeManageJig;

impl Scope for ScopeManageJig {
    fn scope() -> UserScope {
        UserScope::ManageJig
    }
}

#[derive(Apiv2Schema)]
pub struct ScopeManageModule;

impl Scope for ScopeManageModule {
    fn scope() -> UserScope {
        UserScope::ManageModule
    }
}

#[derive(Apiv2Schema)]
pub struct ScopeAdmin;

impl Scope for ScopeAdmin {
    fn scope() -> UserScope {
        UserScope::Admin
    }
}

#[derive(Apiv2Schema)]
pub struct ScopeManageAnimation;

impl Scope for ScopeManageAnimation {
    fn scope() -> UserScope {
        UserScope::ManageAnimation
    }
}

#[derive(Apiv2Security)]
#[openapi(
    apiKey,
    alias = "scopedApiKey",
    in = "header",
    name = "Authorization",
    description = "Use format 'Bearer TOKEN'"
)]
#[repr(transparent)]
pub struct AuthUserWithScope<S: Scope> {
    pub claims: AuthClaims,
    _phantom: PhantomData<S>,
}

impl<S: Scope> FromRequest for AuthUserWithScope<S> {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let cookie = req.cookie(JWT_COOKIE_NAME);
        let csrf = csrf_header(req.headers());
        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");
        let db: &Data<PgPool> = req.app_data().expect("Missing `Data` for db?");
        let db = db.as_ref().clone();

        let (cookie, csrf) = match check_cookie_csrf(cookie, csrf) {
            Ok((cookie, csrf)) => (cookie, csrf),
            Err(e) => return futures::future::err(e.into()).into(),
        };
        // get claims and check csrf
        let claims = check_no_db(cookie.value(), csrf, &settings.jwt_decoding_key())
            .map_err(|_| BasicError::new(StatusCode::UNAUTHORIZED))
            .and_then(|it| it.ok_or_else(|| BasicError::new(StatusCode::UNAUTHORIZED)));

        let claims = match claims {
            Ok(claims) => claims,
            Err(e) => return future::err(e.into()).into(),
        };

        async move {
            let has_scope = sqlx::query!(
                r#"select exists(select 1 from "user_scope" where user_id = $1 and scope = $2) as "exists!""#,
                claims.id,
                S::scope() as i16
            )
            .fetch_one(&db)
            .await
            .map(|it| it.exists)
            .map_err(Into::into)
            .map_err(crate::error::ise)?;

            if !has_scope {
                // todo: message for which scope is needed
                return Err(BasicError::new(StatusCode::FORBIDDEN).into());
            }

            Ok(Self {
                claims,
                _phantom: PhantomData,
            })
        }
        .boxed()
        .into()
    }
}

#[derive(Apiv2Security)]
#[openapi(
    apiKey,
    in = "header",
    name = "Authorization",
    description = "Use format 'Bearer TOKEN'"
)]
#[repr(transparent)]
pub struct WrapAuthClaimsCookieDbNoCsrf(pub AuthClaims);

impl FromRequest for WrapAuthClaimsCookieDbNoCsrf {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let db: &Data<PgPool> = req.app_data().unwrap();
        let db = db.as_ref().clone();
        let settings: &Data<RuntimeSettings> = req.app_data().unwrap();
        let settings = settings.clone();

        let cookie = match req.cookie(JWT_COOKIE_NAME) {
            Some(token) => token.to_owned(),
            None => return future::err(BasicError::new(StatusCode::UNAUTHORIZED).into()).into(),
        };

        async move {
            check_no_csrf(&db, cookie.value(), &settings.jwt_decoding_key())
                .await
                .map_err(crate::error::ise)?
                .map(Self)
                .ok_or_else(|| BasicError::new(StatusCode::UNAUTHORIZED).into())
        }
        .boxed()
        .into()
    }
}

pub fn reply_signin_auth(
    user_id: Uuid,
    jwt_encoding_key: &EncodingKey,
    local_insecure: bool,
) -> anyhow::Result<(String, Cookie<'static>)> {
    let csrf: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

    let claims = AuthClaims {
        id: user_id,
        csrf: Some(csrf.clone()),
    };

    let jwt = jwt::encode(&jwt::Header::default(), &claims, jwt_encoding_key)?;

    let mut cookie = CookieBuilder::new(JWT_COOKIE_NAME, jwt)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(MAX_SIGNIN_COOKIE_DURATION);

    if !local_insecure {
        cookie = cookie.domain(COOKIE_DOMAIN);
    }

    Ok((csrf, cookie.finish()))
}
