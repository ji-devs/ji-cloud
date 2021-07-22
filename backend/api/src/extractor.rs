use crate::{
    domain::RegistrationStatus,
    error::BasicError,
    more_futures::ReadyOrNot,
    token::{check_login_token, SessionClaims, SessionMask},
};
use actix_http::error::BlockingError;
use actix_web::{cookie::Cookie, http::HeaderMap, web::Data, Either, FromRequest, HttpMessage};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use argon2::{
    password_hash::{Encoding, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};
use core::settings::RuntimeSettings;
use futures::future::{self, FutureExt};
use http::StatusCode;
use paperclip::actix::{Apiv2Schema, Apiv2Security};
use rand::thread_rng;
use shared::domain::{
    session::{SessionTokenQuery, AUTH_COOKIE_NAME, CSRF_HEADER_NAME},
    user::UserScope,
};
use sqlx::postgres::PgPool;
use std::{borrow::Cow, marker::PhantomData};
use uuid::Uuid;

fn token_query(query_string: &str) -> Option<String> {
    serde_urlencoded::from_str::<SessionTokenQuery>(query_string)
        .map(|it| it.access_token)
        .unwrap_or(None)
}

fn token_header(headers: &HeaderMap) -> Option<String> {
    let parse_for_token = |header: &str| -> Option<String> {
        let mut it = header.split(" ");

        if let Some(head) = it.next() {
            if !head.eq_ignore_ascii_case("Bearer") {
                return None;
            }
        }

        it.next().map(ToOwned::to_owned)
    };

    headers
        .get(http::header::AUTHORIZATION)?
        .to_str()
        .map_or_else(|_| None, parse_for_token)
}

fn csrf_header(headers: &HeaderMap) -> Option<String> {
    headers
        .get(CSRF_HEADER_NAME)?
        .to_str()
        .ok()
        .map(ToOwned::to_owned)
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
pub struct TokenUser(pub SessionClaims);

impl FromRequest for TokenUser {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");
        let settings = Data::clone(settings);

        let db: &Data<PgPool> = req.app_data().expect("Missing `Data` for db?");
        let db = db.as_ref().clone();

        let token = token_query(req.query_string()).or_else(|| token_header(req.headers()));

        let (token_string, csrf) = match token {
            Some(token_string) => (token_string, None),
            None => {
                let cookie = req.cookie(AUTH_COOKIE_NAME);
                let csrf = csrf_header(req.headers());

                match check_cookie_csrf(cookie, csrf.map(Cow::Owned)) {
                    Ok((cookie, csrf)) => (cookie.value().to_owned(), Some(csrf.into_owned())),
                    Err(e) => return futures::future::err(e.into()).into(),
                }
            }
        };

        async move {
            let csrf = csrf;
            let claims = check_login_token(
                &db,
                &token_string,
                csrf.as_deref(),
                &settings.token_secret,
                SessionMask::GENERAL_API,
            )
            .await?;

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
pub struct ScopeAdminJig;

impl Scope for ScopeAdminJig {
    fn scope() -> UserScope {
        UserScope::AdminJig
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

#[derive(Apiv2Schema)]
pub struct ScopeManageManageEntry;

impl Scope for ScopeManageManageEntry {
    fn scope() -> UserScope {
        UserScope::ManageEntry
    }
}
#[derive(Apiv2Schema)]
pub struct ScopeManageAudio;

impl Scope for ScopeManageAudio {
    fn scope() -> UserScope {
        UserScope::ManageAudio
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
    pub claims: SessionClaims,
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
        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");
        let settings = Data::clone(settings);

        let db: &Data<PgPool> = req.app_data().expect("Missing `Data` for db?");
        let db = db.as_ref().clone();

        let token = token_query(req.query_string()).or_else(|| token_header(req.headers()));

        let (token_string, csrf) = match token {
            Some(token_string) => (token_string, None),
            None => {
                let cookie = req.cookie(AUTH_COOKIE_NAME);
                let csrf = csrf_header(req.headers());

                match check_cookie_csrf(cookie, csrf.map(Cow::Owned)) {
                    Ok((cookie, csrf)) => (cookie.value().to_owned(), Some(csrf.into_owned())),
                    Err(e) => return futures::future::err(e.into()).into(),
                }
            }
        };

        async move {
            let csrf = csrf;
            // todo: fix the race condition here (user deleted between the db access in `check_token` and `has_scope`)
            let claims = check_login_token(
                &db,
                &token_string,
                csrf.as_deref(),
                &settings.token_secret,
                SessionMask::GENERAL_API
            )
                .await?;

            let has_scope = sqlx::query!(
                r#"select exists(select 1 from "user_scope" where user_id = $1 and (scope = $2 or scope = $3)) as "exists!""#,
                claims.user_id,
                S::scope() as i16,
                UserScope::Admin as i16
            )
            .fetch_optional(&db)
            .await
            .map_err(Into::into)
            .map_err(crate::error::ise)?.map(|it| it.exists).ok_or_else(|| BasicError::new(StatusCode::FORBIDDEN))?;

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

pub trait SessionMaskRequirement {
    const REQUIREMENTS: SessionMask;
}

#[derive(Apiv2Schema)]
pub struct SessionPutProfile;

impl SessionMaskRequirement for SessionPutProfile {
    const REQUIREMENTS: SessionMask = SessionMask::PUT_PROFILE;
}

#[derive(Apiv2Schema)]
pub struct SessionChangePassword;

impl SessionMaskRequirement for SessionChangePassword {
    const REQUIREMENTS: SessionMask = SessionMask::CHANGE_PASSWORD;
}

#[derive(Apiv2Schema)]
pub struct SessionVerifyEmail;

impl SessionMaskRequirement for SessionVerifyEmail {
    const REQUIREMENTS: SessionMask = SessionMask::VERIFY_EMAIL;
}

#[derive(Apiv2Schema)]
pub struct SessionAny;

impl SessionMaskRequirement for SessionAny {
    const REQUIREMENTS: SessionMask = SessionMask::empty();
}

#[derive(Apiv2Schema)]
pub struct SessionDelete;

impl SessionMaskRequirement for SessionDelete {
    const REQUIREMENTS: SessionMask = SessionMask::DELETE_ACCOUNT;
}

#[derive(Apiv2Security)]
#[openapi(
    apiKey,
    alias = "temporaryApiKey",
    in = "header",
    name = "Authorization",
    description = "Use format 'Bearer TOKEN'"
)]
#[repr(transparent)]
pub struct TokenSessionOf<S: SessionMaskRequirement> {
    pub claims: SessionClaims,
    _phantom: PhantomData<S>,
}

impl<S: SessionMaskRequirement> FromRequest for TokenSessionOf<S> {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");
        let settings = Data::clone(settings);

        let db: &Data<PgPool> = req.app_data().expect("Missing `Data` for db?");
        let db = db.as_ref().clone();

        let token = token_query(req.query_string()).or_else(|| token_header(req.headers()));

        let (token_string, csrf) = match token {
            Some(token_string) => (token_string, None),
            None => {
                let cookie = req.cookie(AUTH_COOKIE_NAME);
                let csrf = csrf_header(req.headers());

                match check_cookie_csrf(cookie, csrf.map(Cow::Owned)) {
                    Ok((cookie, csrf)) => (cookie.value().to_owned(), Some(csrf.into_owned())),
                    Err(e) => return futures::future::err(e.into()).into(),
                }
            }
        };

        async move {
            let csrf = csrf;
            let claims = check_login_token(
                &db,
                &token_string,
                csrf.as_deref(),
                &settings.token_secret,
                S::REQUIREMENTS,
            )
            .await?;

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
    pub registration_status: RegistrationStatus,
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
                r#"
select
    user_id,
    password,
    exists(select 1 from user_profile where user_id = user_auth_basic.user_id) as "has_profile!",
    exists(select 1 from user_email where user_id = user_auth_basic.user_id) as "has_verified_email!"
from user_auth_basic where email = $1::text
"#,
                &*email
            )
            .fetch_optional(&db)
            .await
            .map_err(Into::into)
            .map_err(crate::error::ise)?;

            actix_web::web::block(move || {
                let password_hasher = Argon2::default();

                let user = match user {
                    Some(user) => user,
                    None => {
                        let salt = SaltString::generate(thread_rng());
                        let _ = password_hasher.hash_password(
                            password.as_bytes(),
                            None,
                            None,
                            crate::ARGON2_DEFAULT_PARAMS,
                            salt.as_salt(),
                        );

                        return Err(Either::A(BasicError::new(StatusCode::UNAUTHORIZED)));
                    }
                };

                let hash = argon2::PasswordHash::parse(&user.password, Encoding::default())
                    .map_err(|it| Either::B(anyhow::anyhow!("{}", it)))?;

                password_hasher
                    .verify_password(password.as_bytes(), &hash)
                    .map_err(|_| Either::A(BasicError::new(StatusCode::UNAUTHORIZED)))?;

                let registration_status = match (user.has_verified_email, user.has_profile) {
                    // todo: "???"
                    (false, _) => return Err(Either::A(BasicError::new(StatusCode::FORBIDDEN))),
                    // (false, _) => RegistrationStatus::New,
                    (true, false) => RegistrationStatus::Validated,
                    (true, true) => RegistrationStatus::Complete,
                };

                Ok(Self { id: user.user_id, registration_status })
            })
            .await
            .map_err(blocking_error)
        }
        .boxed()
        .into()
    }
}

fn blocking_error(err: BlockingError<Either<BasicError, anyhow::Error>>) -> actix_web::Error {
    match err {
        BlockingError::Canceled => crate::error::ise(anyhow::anyhow!("Thread pool is gone")),
        BlockingError::Error(Either::B(e)) => crate::error::ise(e),
        BlockingError::Error(Either::A(e)) => e.into(),
    }
}
