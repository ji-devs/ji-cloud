use crate::{
    db::{self, user::upsert_profile},
    error,
    extractor::{SessionPurposeCreateProfile, TokenUserWithPurposedSession},
};
use crate::{extractor::TokenUser, token::create_auth_token};
use actix_web::HttpResponse;
use chrono::{Duration, Utc};
use core::settings::RuntimeSettings;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Query, ServiceConfig},
};
use shared::{
    api::endpoints::{
        user::{Profile, PutProfile, UserLookup},
        ApiEndpoint,
    },
    domain::{
        session::NewSessionResponse,
        user::{PutProfileRequest, UserLookupQuery},
    },
    error::auth::RegisterErrorKind,
};
use sqlx::{Acquire, PgPool};

/// Lookup a user.
#[api_v2_operation]
async fn user_lookup(
    db: Data<PgPool>,
    query: Query<UserLookupQuery>,
) -> Result<Json<<UserLookup as ApiEndpoint>::Res>, error::UserNotFound> {
    let query = query.into_inner();

    db::user::lookup(db.as_ref(), query.id, query.name.as_deref())
        .await?
        .map(Json)
        .ok_or(error::UserNotFound::UserNotFound)
}

async fn validate_register_req(req: &PutProfileRequest) -> Result<(), error::Register> {
    if req.username.is_empty() {
        return Err(error::Register::RegisterError(
            RegisterErrorKind::EmptyDisplayName,
        ));
    }

    Ok(())
}

/// Create or replace your profile.
#[api_v2_operation]
async fn handle_put_profile(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    signup_user: TokenUserWithPurposedSession<SessionPurposeCreateProfile>,
    req: Json<PutProfileRequest>,
) -> actix_web::Result<HttpResponse, error::Register> {
    validate_register_req(&req).await?;

    let mut txn = db.begin().await?;

    let mut upsert_txn = txn.begin().await?;

    upsert_profile(&mut upsert_txn, &req, signup_user.claims.user_id).await?;

    upsert_txn.commit().await?;

    let session = crate::token::generate_session_token();

    let login_ttl = settings
        .login_token_valid_duration
        .unwrap_or(Duration::weeks(2));

    db::session::create_new(
        &mut txn,
        signup_user.claims.user_id,
        &session,
        Some(&(Utc::now() + login_ttl)),
        None,
        None,
    )
    .await?;

    let (csrf, cookie) = create_auth_token(
        &settings.token_secret,
        settings.is_local(),
        login_ttl,
        &session,
    )?;

    txn.commit().await?;

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(NewSessionResponse { csrf }))
}

/// Get a user's profile.
#[api_v2_operation]
async fn get_profile(
    db: Data<PgPool>,
    claims: TokenUser,
) -> Result<Json<<Profile as ApiEndpoint>::Res>, error::UserNotFound> {
    // todo: figure out how to do `<Profile as ApiEndpoint>::Err`

    db::user::profile(db.as_ref(), claims.0.user_id)
        .await?
        .map(Json)
        .ok_or(error::UserNotFound::UserNotFound)
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(Profile::PATH, Profile::METHOD.route().to(get_profile))
        .route(
            PutProfile::PATH,
            PutProfile::METHOD.route().to(handle_put_profile),
        )
        .route(UserLookup::PATH, UserLookup::METHOD.route().to(user_lookup));
}
