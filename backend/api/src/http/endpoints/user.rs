use crate::{
    db::{self, user::upsert_profile},
    error,
    extractor::{SessionPurposeCreateProfile, TokenUserWithPurposedSession},
    service::{mail, ServiceData},
    token::TokenPurpose,
};
use crate::{extractor::TokenUser, token::create_auth_token};
use actix_http::error::BlockingError;
use actix_web::HttpResponse;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{Duration, Utc};
use core::settings::RuntimeSettings;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Query, ServiceConfig},
    NoContent,
};
use rand::thread_rng;
use sendgrid::v3::Email;
use shared::{
    api::endpoints::{
        user::{Create, Profile, PutProfile, UserLookup, VerifyEmail},
        ApiEndpoint,
    },
    domain::{
        session::NewSessionResponse,
        user::{PutProfileRequest, UserLookupQuery, VerifyEmailRequest},
    },
};
use sqlx::{Acquire, PgPool};

/// Create a user
#[api_v2_operation]
async fn create_user(
    config: Data<RuntimeSettings>,
    mail: ServiceData<mail::Client>,
    db: Data<PgPool>,
    req: Json<<Create as ApiEndpoint>::Req>,
) -> Result<NoContent, error::Service> {
    let req = req.into_inner();

    if req.password.is_empty() {
        todo!("empty password is error");
    }

    let mut txn = db.begin().await?;

    // todo: handle duplicate emails

    let user = sqlx::query!(r#"insert into "user" default values returning id"#)
        .fetch_one(&mut txn)
        .await?;

    let pass = req.password;

    let res = actix_web::web::block(move || {
        let password_hasher = Argon2::default();

        let salt = SaltString::generate(thread_rng());
        password_hasher
            .hash_password(
                pass.as_bytes(),
                None,
                None,
                crate::ARGON2_DEFAULT_PARAMS,
                salt.as_salt(),
            )
            .map_err(|it| anyhow::anyhow!("{}", it))
            .map(|it| it.to_string())
    })
    .await;

    let pass_hash = match res {
        Ok(hash) => hash,
        Err(BlockingError::Canceled) => return Err(anyhow::anyhow!("Thread pool is gone").into()),
        Err(BlockingError::Error(e)) => return Err(anyhow::anyhow!("{}", e).into()),
    };

    sqlx::query!(
        "insert into user_auth_basic (user_id, email, password) values ($1, $2::text, $3)",
        user.id,
        &req.email,
        pass_hash.to_string(),
    )
    .execute(&mut txn)
    .await?;

    let session = db::session::create(
        &mut txn,
        user.id,
        Some(&(Utc::now() + Duration::hours(1))),
        Some(TokenPurpose::VerifyEmail),
        None,
    )
    .await?;

    txn.commit().await?;

    let template = mail
        .signup_verify_template()
        .map_err(error::Service::DisabledService)?;

    let email_link = format!(
        "{}/verify-email/{}",
        config.remote_target().pages_url(),
        session
    );

    mail.send_signup_verify(template, Email::new(req.email), email_link)
        .await?;

    Ok(NoContent)
}

/// Verify emails
#[api_v2_operation]
async fn verify_email(
    config: Data<RuntimeSettings>,
    mail: ServiceData<mail::Client>,
    db: Data<PgPool>,
    req: Json<<VerifyEmail as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Service> {
    let req = req.into_inner();

    match req {
        VerifyEmailRequest::Resend { email } => {
            let mut txn = db.begin().await?;

            // todo: make this more future proof and exhaustive.

            let user = sqlx::query!(
                r#"
select user_id
from user_auth_basic
where
    email = $1::text and
    not exists(select 1 from user_email where email = $1)
"#,
                email
            )
            .fetch_optional(&mut txn)
            .await?;

            let user = match user {
                Some(user) => user,
                None => return Ok(HttpResponse::NoContent().into()),
            };

            // make sure they can't use the old link anymore
            sqlx::query!(
                "delete from session where user_id = $1 and scope = $2",
                user.user_id,
                TokenPurpose::VerifyEmail as i16,
            )
            .execute(&mut txn)
            .await?;

            let session = db::session::create(
                &mut txn,
                user.user_id,
                Some(&(Utc::now() + Duration::hours(1))),
                Some(TokenPurpose::VerifyEmail),
                None,
            )
            .await?;

            txn.commit().await?;

            let template = mail
                .signup_verify_template()
                .map_err(error::Service::DisabledService)?;

            let email_link = format!(
                "{}/verify-email/{}",
                config.remote_target().pages_url(),
                session
            );

            mail.send_signup_verify(template, Email::new(email), email_link)
                .await?;

            Ok(HttpResponse::NoContent().into())
        }

        VerifyEmailRequest::Verify { token } => {
            let mut txn = db.begin().await?;

            // todo: make this more future proof and exhaustive.

            let user = sqlx::query!(
                r#"
insert into user_email (user_id, email)
select session.user_id, user_auth_basic.email
from session
inner join user_auth_basic on user_auth_basic.user_id = session.user_id
where 
    session.token = $1 and
    session.expires_at > now() and
    session.scope is not distinct from $2
returning user_id
"#,
                token,
                TokenPurpose::VerifyEmail as i16,
            )
            .fetch_optional(&mut txn)
            .await?
            .ok_or_else(|| anyhow::anyhow!("this should 401"))?;

            // make sure they can't use the old link anymore
            sqlx::query!(
                "delete from session where user_id = $1 and scope = $2",
                user.user_id,
                TokenPurpose::VerifyEmail as i16,
            )
            .execute(&mut txn)
            .await?;

            let purpose = TokenPurpose::CreateProfile;

            let login_ttl = config
                .login_token_valid_duration
                .unwrap_or(Duration::weeks(2));

            let valid_until = Utc::now() + Duration::hours(1);

            crate::token::generate_session_token();

            let session = db::session::create(
                &mut txn,
                user.user_id,
                Some(&valid_until),
                Some(purpose),
                None,
            )
            .await?;

            txn.commit().await?;

            let (csrf, cookie) =
                create_auth_token(&config.token_secret, config.is_local(), login_ttl, &session)?;

            Ok(HttpResponse::Created()
                .cookie(cookie)
                .json(NewSessionResponse { csrf }))
        }
    }
}

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

fn validate_register_req(req: &PutProfileRequest) -> Result<(), error::Register> {
    if req.username.is_empty() {
        return Err(error::Register::EmptyUsername);
    }

    Ok(())
}

/// Create or replace your profile.
#[api_v2_operation]
async fn put_profile(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    signup_user: TokenUserWithPurposedSession<SessionPurposeCreateProfile>,
    req: Json<PutProfileRequest>,
) -> actix_web::Result<HttpResponse, error::Register> {
    validate_register_req(&req)?;

    let mut txn = db.begin().await?;

    let mut upsert_txn = txn.begin().await?;

    upsert_profile(&mut upsert_txn, &req, signup_user.claims.user_id).await?;

    upsert_txn.commit().await?;

    let session = crate::token::generate_session_token();

    let login_ttl = settings
        .login_token_valid_duration
        .unwrap_or(Duration::weeks(2));

    db::session::create_with_token(
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
        .route(Create::PATH, Create::METHOD.route().to(create_user))
        .route(
            VerifyEmail::PATH,
            VerifyEmail::METHOD.route().to(verify_email),
        )
        .route(PutProfile::PATH, PutProfile::METHOD.route().to(put_profile))
        .route(UserLookup::PATH, UserLookup::METHOD.route().to(user_lookup));
}
