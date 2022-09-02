use actix_web::{
    web::{Data, ServiceConfig},
    HttpResponse,
};
use chrono::{Duration, Utc};
use core::settings::RuntimeSettings;
use shared::{
    api::{endpoints::session, ApiEndpoint, PathParts},
    domain::session::{CreateSessionResponse, NewSessionResponse},
};
use sqlx::PgPool;

use crate::{
    db,
    domain::{NoContentClearAuth, RegistrationStatus},
    error,
    extractor::{EmailBasicUser, SessionAny, TokenSessionOf},
    token::{create_auth_token, SessionMask},
};

mod oauth;

/// Login with basic authorization.
/// May return resources for *signing up* if the user doesn't have a profile.
async fn create_session(
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    user: EmailBasicUser,
) -> Result<HttpResponse, error::Server> {
    let login_ttl = settings
        .login_token_valid_duration
        .unwrap_or(Duration::weeks(2));

    let (mask, valid_until) = match user.registration_status {
        RegistrationStatus::New => panic!("This isn't currently possible"),
        RegistrationStatus::Validated => (
            SessionMask::PUT_PROFILE | SessionMask::DELETE_ACCOUNT,
            Utc::now() + Duration::hours(1),
        ),
        RegistrationStatus::Complete => (SessionMask::GENERAL, Utc::now() + login_ttl),
    };

    let mut txn = db.begin().await?;

    let session = db::session::create(&mut txn, user.id, Some(&valid_until), mask, None).await?;

    let (csrf, cookie) = create_auth_token(
        &settings.token_secret,
        settings.is_local(),
        login_ttl,
        &session,
    )?;

    txn.commit().await?;

    let response = NewSessionResponse { csrf };

    let response = if !mask.contains(SessionMask::GENERAL) {
        CreateSessionResponse::Register {
            response,
            oauth_profile: None,
        }
    } else {
        CreateSessionResponse::Login(response)
    };

    Ok(HttpResponse::Created().cookie(cookie).json(response))
}

/// Logout
async fn delete_session(
    db: Data<PgPool>,
    session: TokenSessionOf<SessionAny>,
) -> Result<NoContentClearAuth, error::Server> {
    sqlx::query!("delete from session where token = $1", session.claims.token)
        .execute(db.as_ref())
        .await?;

    Ok(NoContentClearAuth)
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <session::GetOAuthUrl as ApiEndpoint>::Path::PATH,
        session::GetOAuthUrl::METHOD.route().to(oauth::get_url),
    )
    .route(
        <session::Create as ApiEndpoint>::Path::PATH,
        session::Create::METHOD.route().to(create_session),
    )
    .route(
        <session::Delete as ApiEndpoint>::Path::PATH,
        session::Delete::METHOD.route().to(delete_session),
    )
    .route(
        <session::CreateOAuth as ApiEndpoint>::Path::PATH,
        session::CreateOAuth::METHOD.route().to(oauth::create),
    );
}
