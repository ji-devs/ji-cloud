use crate::{
    error::BasicError,
    jwt::{check_token, TokenClaims, TokenSource},
    more_futures::ReadyOrNot,
};
use actix_web::{
    cookie::{Cookie, CookieBuilder, SameSite},
    http::HeaderMap,
    web::Data,
    FromRequest, HttpMessage,
};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use argon2::{password_hash::Encoding, Argon2, PasswordHasher, PasswordVerifier, Version};
use chrono::{Duration, Utc};
use config::{COOKIE_DOMAIN, MAX_SIGNIN_COOKIE_DURATION};
use core::settings::RuntimeSettings;
use futures::future::{self, FutureExt};
use http::StatusCode;
use paperclip::actix::{Apiv2Schema, Apiv2Security};
use paseto::PasetoBuilder;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use shared::domain::{
    auth::{AUTH_COOKIE_NAME, CSRF_HEADER_NAME},
    user::UserScope,
};
use sqlx::postgres::PgPool;
use std::{borrow::Cow, marker::PhantomData};
use uuid::Uuid;

fn csrf_header(headers: &HeaderMap) -> Option<&str> {
    headers.get(CSRF_HEADER_NAME)?.to_str().ok()
}

fn check_cookie_csrf<'a>(
    cookie: Option<Cookie<'a>>,
    csrf: Option<Cow<'a, str>>,
) -> Result<(Cookie<'a>, Cow<'a, str>), BasicError> {
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
pub struct TokenUser(pub TokenClaims);

impl FromRequest for TokenUser {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let cookie = req.cookie(AUTH_COOKIE_NAME);
        let csrf = csrf_header(req.headers()).map(ToOwned::to_owned);

        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");
        let settings = Data::clone(settings);

        let db: &Data<PgPool> = req.app_data().expect("Missing `Data` for db?");
        let db = db.as_ref().clone();

        let (cookie, csrf) = match check_cookie_csrf(cookie, csrf.map(Cow::Owned)) {
            Ok((cookie, csrf)) => (cookie, csrf.into_owned()),
            Err(e) => return futures::future::err(e.into()).into(),
        };

        async move {
            let csrf = csrf;
            // todo: fix the race condition here (user deleted between the db access in `check_token` and `has_scope`)
            let claims = check_token(&db, cookie.value(), &csrf, &settings.token_secret).await?;

            Ok(Self(claims))
        }
        .boxed()
        .into()
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
pub struct TokenUserWithScope<S: Scope> {
    pub claims: TokenClaims,
    _phantom: PhantomData<S>,
}

impl<S: Scope> FromRequest for TokenUserWithScope<S> {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let cookie = req.cookie(AUTH_COOKIE_NAME);
        let csrf = csrf_header(req.headers()).map(ToOwned::to_owned);

        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");
        let settings = Data::clone(settings);

        let db: &Data<PgPool> = req.app_data().expect("Missing `Data` for db?");
        let db = db.as_ref().clone();

        let (cookie, csrf) = match check_cookie_csrf(cookie, csrf.map(Cow::Owned)) {
            Ok((cookie, csrf)) => (cookie, csrf.into_owned()),
            Err(e) => return futures::future::err(e.into()).into(),
        };

        async move {
            let csrf = csrf;
            // todo: fix the race condition here (user deleted between the db access in `check_token` and `has_scope`)
            let claims = check_token(&db, cookie.value(), &csrf, &settings.token_secret).await?;

            let has_scope = sqlx::query!(
                r#"select exists(select 1 from "user_scope" where user_id = $1 and scope = $2) as "exists!""#,
                claims.sub,
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
    alias = "BasicApi",
    in = "header",
    name = "Authorization",
    description = "Use format 'Basic email:password'"
)]
pub struct EmailBasicUser {
    pub id: Uuid,
}

impl FromRequest for EmailBasicUser {
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

        let basic = match req.get_header::<Authorization<Basic>>() {
            Some(basic) => basic.into_scheme(),
            None => return future::err(BasicError::new(StatusCode::UNAUTHORIZED).into()).into(),
        };

        let (email, password) = match basic.password() {
            Some(password) => (Cow::clone(basic.user_id()), Cow::clone(password)),
            None => return future::err(BasicError::new(StatusCode::UNAUTHORIZED).into()).into(),
        };

        async move {
            let user = sqlx::query!(
                "select user_id, password from user_auth_basic where email = $1::text",
                &*email
            )
            .fetch_optional(&db)
            .await
            .map_err(Into::into)
            .map_err(crate::error::ise)?;

            let password_hasher = Argon2::new(
                Some(settings.password_secret.as_bytes()),
                3,
                4096,
                1,
                Version::default(),
            )
            .expect("Mostly default argon2 settings");

            let user = match user {
                Some(user) => user,
                None => {
                    // we don't care about the results of this
                    let _ = password_hasher.hash_password_simple(password.as_bytes(), "");
                    return Err(BasicError::new(StatusCode::UNAUTHORIZED).into());
                }
            };

            let hash = argon2::PasswordHash::parse(&user.password, Encoding::default())
                .map_err(|it| crate::error::ise(anyhow::anyhow!("{}", it)))?;

            password_hasher
                .verify_password(password.as_bytes(), &hash)
                .map_err(|_| BasicError::new(StatusCode::UNAUTHORIZED))?;

            Ok(Self { id: user.user_id })
        }
        .boxed()
        .into()
    }
}

pub fn reply_signin_auth(
    user_id: Uuid,
    token_secret: &[u8; 32],
    local_insecure: bool,
    source: TokenSource,
) -> anyhow::Result<(String, Cookie<'static>)> {
    let csrf: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

    let token = PasetoBuilder::new()
        .set_subject(&user_id.to_hyphenated().to_string())
        .set_issued_at(None)
        .set_expiration(&(Utc::now() + Duration::weeks(2)))
        .set_encryption_key(token_secret)
        .set_claim("csrf", serde_json::Value::String(csrf.clone()))
        .set_claim("source", serde_json::to_value(source)?)
        .build()
        .map_err(|err| anyhow::anyhow!("failed to create token: {}", err))?;

    let mut cookie = CookieBuilder::new(AUTH_COOKIE_NAME, token)
        .http_only(true)
        .secure(!local_insecure)
        .same_site(SameSite::Lax)
        .max_age(MAX_SIGNIN_COOKIE_DURATION);

    if !local_insecure {
        cookie = cookie.domain(COOKIE_DOMAIN);
    }

    Ok((csrf, cookie.finish()))
}
