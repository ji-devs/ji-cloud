use crate::{
    domain::RegistrationStatus,
    error::BasicError,
    more_futures::ReadyOrNot,
    token::{check_login_token, SessionClaims, SessionMask},
};

use actix_http::Payload;
use actix_web::{
    cookie::Cookie,
    http::{header, header::HeaderMap},
    web::Data,
    Either, FromRequest, HttpMessage, HttpRequest,
};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use argon2::{
    password_hash::{Encoding, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};
use core::settings::RuntimeSettings;
use futures::future::{self, ready, FutureExt};
use http::StatusCode;
use rand::thread_rng;
use shared::domain::{
    session::{SessionTokenQuery, AUTH_COOKIE_NAME, CSRF_HEADER_NAME},
    user::{UserId, UserScope},
};
use sqlx::postgres::PgPool;
use std::{borrow::Cow, marker::PhantomData};

fn token_from_query(query_string: &str) -> Option<String> {
    serde_urlencoded::from_str::<SessionTokenQuery>(query_string)
        .map(|it| it.access_token)
        .unwrap_or(None)
}

fn token_from_header(headers: &HeaderMap) -> Option<String> {
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

async fn claims_for_scope(
    db: &PgPool,
    settings: &Data<RuntimeSettings>,
    token_string: &str,
    csrf: Option<&str>,
    user_scope: UserScope,
) -> Result<SessionClaims, actix_web::Error> {
    let claims = check_login_token(
        &db,
        &token_string,
        csrf,
        &settings.token_secret,
        SessionMask::GENERAL_API,
    )
    .await?;

    let has_scope = sqlx::query!(
        r#"select exists(
            select 1
            from "user_scope"
            where
                user_id = $1 and
                (scope = $2 or scope = $3)
        ) as "exists!""#,
        claims.user_id,
        user_scope as i16,
        UserScope::Admin as i16
    )
    .fetch_optional(db)
    .await
    .map_err(Into::into)
    .map_err(crate::error::ise)?
    .map(|it| it.exists)
    .ok_or_else(|| BasicError::new(StatusCode::FORBIDDEN))?;

    if !has_scope {
        // todo: message for which scope is needed
        return Err(BasicError::new(StatusCode::FORBIDDEN).into());
    }

    Ok(claims)
}

#[repr(transparent)]
pub struct TokenUser(pub SessionClaims);

impl TokenUser {
    /// check if jig
    pub fn user_id(&self) -> UserId {
        UserId(self.0.user_id)
    }
}

impl FromRequest for TokenUser {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");
        let settings = Data::clone(settings);

        let db: &Data<PgPool> = req.app_data().expect("Missing `Data` for db?");
        let db = db.as_ref().clone();

        let token =
            token_from_query(req.query_string()).or_else(|| token_from_header(req.headers()));

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

pub struct ScopeManageCategory;

impl Scope for ScopeManageCategory {
    fn scope() -> UserScope {
        UserScope::ManageCategory
    }
}

pub struct ScopeManageImage;

impl Scope for ScopeManageImage {
    fn scope() -> UserScope {
        UserScope::ManageImage
    }
}

pub struct ScopeAdminJig;

impl Scope for ScopeAdminJig {
    fn scope() -> UserScope {
        UserScope::AdminJig
    }
}

pub struct ScopeAdmin;

impl Scope for ScopeAdmin {
    fn scope() -> UserScope {
        UserScope::Admin
    }
}

pub struct ScopeManageAnimation;

impl Scope for ScopeManageAnimation {
    fn scope() -> UserScope {
        UserScope::ManageAnimation
    }
}

pub struct ScopeManageManageEntry;

impl Scope for ScopeManageManageEntry {
    fn scope() -> UserScope {
        UserScope::ManageEntry
    }
}
pub struct ScopeManageAudio;

impl Scope for ScopeManageAudio {
    fn scope() -> UserScope {
        UserScope::ManageAudio
    }
}

#[repr(transparent)]
pub struct TokenUserWithScope<S: Scope> {
    pub claims: SessionClaims,
    _phantom: PhantomData<S>,
}

impl<S: Scope> FromRequest for TokenUserWithScope<S> {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");
        let settings = Data::clone(settings);

        let db: &Data<PgPool> = req.app_data().expect("Missing `Data` for db?");
        let db = db.as_ref().clone();

        let token =
            token_from_query(req.query_string()).or_else(|| token_from_header(req.headers()));

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
            let claims =
                claims_for_scope(&db, &settings, &token_string, csrf.as_deref(), S::scope())
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

/// Special extractor which doesn't compare the user's CSRF token.1
///
/// **Note** that this extractor should only be used on GET requests which have _no_ side-effects.
#[repr(transparent)]
pub struct TokenUserNoCsrfWithScope<S: Scope> {
    pub claims: SessionClaims,
    _phantom: PhantomData<S>,
}

impl<S: Scope> FromRequest for TokenUserNoCsrfWithScope<S> {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");
        let settings = Data::clone(settings);

        let db: &Data<PgPool> = req.app_data().expect("Missing `Data` for db?");
        let db = db.as_ref().clone();

        let cookie = req.cookie(AUTH_COOKIE_NAME);

        let token_string = match cookie {
            Some(cookie) => cookie.value().to_owned(),
            None => {
                return futures::future::err(
                    BasicError::with_message(
                        StatusCode::UNAUTHORIZED,
                        "Unauthorized: missing cookie".to_owned(),
                    )
                    .into(),
                )
                .into()
            }
        };

        async move {
            let claims = claims_for_scope(&db, &settings, &token_string, None, S::scope()).await?;

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

pub struct SessionCreateProfile;
impl SessionMaskRequirement for SessionCreateProfile {
    const REQUIREMENTS: SessionMask = SessionMask::PUT_PROFILE;
}

pub struct SessionChangePassword;
impl SessionMaskRequirement for SessionChangePassword {
    const REQUIREMENTS: SessionMask = SessionMask::CHANGE_PASSWORD;
}

pub struct SessionVerifyEmail;
impl SessionMaskRequirement for SessionVerifyEmail {
    const REQUIREMENTS: SessionMask = SessionMask::VERIFY_EMAIL;
}

pub struct SessionAny;
impl SessionMaskRequirement for SessionAny {
    const REQUIREMENTS: SessionMask = SessionMask::empty();
}

pub struct SessionDelete;
impl SessionMaskRequirement for SessionDelete {
    const REQUIREMENTS: SessionMask = SessionMask::DELETE_ACCOUNT;
}

#[repr(transparent)]
pub struct TokenSessionOf<S: SessionMaskRequirement> {
    pub claims: SessionClaims,
    _phantom: PhantomData<S>,
}

impl<S: SessionMaskRequirement> FromRequest for TokenSessionOf<S> {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let settings: &Data<RuntimeSettings> = req.app_data().expect("Settings??");
        let settings = Data::clone(settings);

        let db: &Data<PgPool> = req.app_data().expect("Missing `Data` for db?");
        let db = db.as_ref().clone();

        let token =
            token_from_query(req.query_string()).or_else(|| token_from_header(req.headers()));

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

pub struct EmailBasicUser {
    pub id: UserId,
    pub registration_status: RegistrationStatus,
}

impl FromRequest for EmailBasicUser {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
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

        let basic_clone = basic.clone();

        let (email, password) = match basic.password() {
            Some(password) => (basic_clone.user_id().to_owned(), password.to_owned()),
            None => return future::err(BasicError::new(StatusCode::UNAUTHORIZED).into()).into(),
        };

        async move {
            let user = sqlx::query!(
                r#"
select
    user_id     as "id!: UserId",
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

            // FIXME handle nested error result with error enum?
            let res = actix_web::web::block(move || -> Result<Self, Either<BasicError, anyhow::Error>> {
                let password_hasher = Argon2::default();

                let user = match user {
                    Some(user) => user,
                    None => {
                        let salt = SaltString::generate(thread_rng());
                        let _ = password_hasher.hash_password(
                            password.as_bytes(),
                            &salt.as_salt(),
                        );

                        return Err(Either::Left(BasicError::new(StatusCode::UNAUTHORIZED)));
                    }
                };

                let hash = match argon2::PasswordHash::parse(&user.password, Encoding::default()) {
                    Ok(hash) => hash,
                    Err(err) => return Err(Either::Right(anyhow::anyhow!("{}", err))),
                };

                password_hasher
                    .verify_password(password.as_bytes(), &hash)
                    .map_err(|_| Either::Left(BasicError::new(StatusCode::UNAUTHORIZED)))?;

                let registration_status = match (user.has_verified_email, user.has_profile) {
                    // todo: "???"
                    (false, _) => return Err(Either::Left(BasicError::new(StatusCode::FORBIDDEN))),
                    // (false, _) => RegistrationStatus::New,
                    (true, false) => RegistrationStatus::Validated,
                    (true, true) => RegistrationStatus::Complete,
                };

                Ok(Self { id: user.id, registration_status })
            })
            .await;

            match res {
                Ok(Ok(res)) => Ok(res),
                Ok(Err(Either::Right(e))) => Err(crate::error::ise(e)),
                Ok(Err(Either::Left(e))) => Err(e.into()),
                Err(e) => Err(crate::error::ise(anyhow::anyhow!("{}", e))),
            }
        }
        .boxed()
        .into()
    }
}

pub struct RequestOrigin(pub Option<String>);

impl FromRequest for RequestOrigin {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let origin = req
            .headers()
            .get(header::ORIGIN)
            .and_then(|it| it.to_str().ok())
            .map(ToOwned::to_owned);

        ready(Ok(Self(origin))).into()
    }
}

pub struct UserAgent(pub Option<String>);

impl FromRequest for UserAgent {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let user_agent = req
            .headers()
            .get(header::USER_AGENT)
            .and_then(|it| it.to_str().ok())
            .map(ToOwned::to_owned);

        ready(Ok(Self(user_agent))).into()
    }
}

pub struct IPAddress(pub Option<String>);

impl FromRequest for IPAddress {
    type Error = actix_web::Error;
    type Future = ReadyOrNot<'static, Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let ip_address = req
            .connection_info()
            .realip_remote_addr()
            .map(|s| s.to_string());

        ready(Ok(Self(ip_address))).into()
    }
}
