use crate::{
    db::{self, user::upsert_profile},
    domain::NoContentClearAuth,
    error,
    extractor::{SessionDelete, SessionPutProfile, TokenSessionOf},
    service::{mail, ServiceData},
    token::SessionMask,
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
        user::{
            ChangePassword, Create, CreateColor, CreateFont, Delete, DeleteColor, DeleteFont,
            GetColors, GetFonts, Profile, PutProfile, ResetPassword, UpdateColor, UpdateFont,
            UserLookup, VerifyEmail,
        },
        ApiEndpoint,
    },
    domain::{
        session::NewSessionResponse,
        user::{ChangePasswordRequest, PutProfileRequest, UserLookupQuery, VerifyEmailRequest},
    },
};
use sqlx::postgres::PgDatabaseError;
use sqlx::{Acquire, PgConnection, PgPool};
use uuid::Uuid;

mod color;
mod font;

async fn send_verification_email(
    txn: &mut PgConnection,
    user_id: Uuid,
    email_address: String,
    mail: &mail::Client,
    pages_url: &str,
) -> Result<(), error::Service> {
    let session = db::session::create(
        &mut *txn,
        user_id,
        Some(&(Utc::now() + Duration::hours(1))),
        SessionMask::VERIFY_EMAIL,
        None,
    )
    .await?;

    let template = mail
        .signup_verify_template()
        .map_err(error::Service::DisabledService)?;

    let email_link = format!("{}/user/verify-email/{}", pages_url, session);

    mail.send_signup_verify(template, Email::new(email_address), email_link)
        .await?;

    Ok(())
}

async fn send_password_email(
    txn: &mut PgConnection,
    user_id: Uuid,
    email_address: String,
    mail: &mail::Client,
    pages_url: &str,
) -> Result<(), error::Service> {
    let session = db::session::create(
        &mut *txn,
        user_id,
        Some(&(Utc::now() + Duration::hours(1))),
        SessionMask::CHANGE_PASSWORD,
        None,
    )
    .await?;

    let template = mail
        .password_reset_template()
        .map_err(error::Service::DisabledService)?;

    let email_link = format!("{}/user/change-pw/{}", pages_url, session);

    mail.send_password_reset(template, Email::new(email_address), email_link)
        .await?;

    Ok(())
}

async fn hash_password(pass: String) -> anyhow::Result<String> {
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

    Ok(pass_hash)
}

/// Create a user
#[api_v2_operation]
async fn create_user(
    config: Data<RuntimeSettings>,
    mail: ServiceData<mail::Client>,
    db: Data<PgPool>,
    req: Json<<Create as ApiEndpoint>::Req>,
) -> Result<NoContent, error::Register> {
    let req = req.into_inner();

    if req.password.is_empty() {
        return Err(anyhow::anyhow!("properly handle empty password error").into());
    }

    let mut txn = db.begin().await?;

    // FIXME simplify these queries
    let exists_basic = sqlx::query!(
        r#"select exists(select 1 from user_auth_basic where email = lower($1)) as "exists!""#,
        &req.email
    )
    .fetch_one(&mut txn)
    .await?
    .exists;
    let exists_google = sqlx::query!(
        r#"select exists(select 1 from user_email where email = lower($1)) as "exists!""#,
        &req.email
    )
    .fetch_one(&mut txn)
    .await?
    .exists;
    match (exists_basic, exists_google) {
        (true, _) => {
            txn.rollback().await?;
            return Err(error::Email::TakenEmailBasic.into());
        }
        (false, true) => {
            txn.rollback().await?;
            return Err(error::Email::TakenEmailGoogle.into());
        }
        (false, false) => (), // do nothing
    }

    let user = sqlx::query!(r#"insert into "user" default values returning id"#)
        .fetch_one(&mut txn)
        .await?;

    let pass_hash = hash_password(req.password).await?;

    sqlx::query!(
        "insert into user_auth_basic (user_id, email, password) values ($1, $2::text, $3)",
        user.id,
        &req.email,
        pass_hash.to_string(),
    )
    .execute(&mut txn)
    .await?;

    send_verification_email(
        &mut txn,
        user.id,
        req.email,
        &mail,
        &config.remote_target().pages_url(),
    )
    .await
    .map_err(error::Register::from)?;

    txn.commit().await?;

    Ok(NoContent)
}

/// Verify emails
#[api_v2_operation]
async fn verify_email(
    config: Data<RuntimeSettings>,
    mail: ServiceData<mail::Client>,
    db: Data<PgPool>,
    req: Json<<VerifyEmail as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::VerifyEmail> {
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
            db::session::clear_any(&mut txn, user.user_id, SessionMask::VERIFY_EMAIL).await?;

            send_verification_email(
                &mut txn,
                user.user_id,
                email,
                &mail,
                &config.remote_target().pages_url(),
            )
            .await
            .map_err(|it| match it {
                error::Service::InternalServerError(it) => {
                    error::VerifyEmail::InternalServerError(it)
                }
                error::Service::DisabledService(it) => {
                    error::ServiceSession::DisabledService(it).into()
                }
            })?;

            txn.commit().await?;

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
    (session.scope_mask & $2) = $2
returning user_id
"#,
                token,
                SessionMask::VERIFY_EMAIL.bits(),
            )
            .fetch_optional(&mut txn) // Result<Option<UserId>, Error>
            .await
            .map_err(|err| match err {
                sqlx::Error::Database(err)
                    if err.downcast_ref::<PgDatabaseError>().constraint()
                        == Some("user_email_email_key") =>
                {
                    error::VerifyEmail::Email(error::Email::TakenEmailBasic)
                }
                err => err.into(),
            })?
            .ok_or(error::VerifyEmail::ServiceSession(
                error::ServiceSession::Unauthorized,
            ))?;

            // make sure they can't use the link, now that they're verified.
            db::session::clear_any(&mut txn, user.user_id, SessionMask::VERIFY_EMAIL).await?;

            let login_ttl = config
                .login_token_valid_duration
                .unwrap_or(Duration::weeks(2));

            let valid_until = Utc::now() + Duration::hours(1);

            let session = db::session::create(
                &mut txn,
                user.user_id,
                Some(&valid_until),
                SessionMask::PUT_PROFILE,
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

fn validate_register_req(req: &PutProfileRequest) -> Result<(), error::RegisterUsername> {
    if req.username.is_empty() {
        return Err(error::RegisterUsername::EmptyUsername);
    }

    Ok(())
}

/// Create or replace your profile.
#[api_v2_operation]
async fn put_profile(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    signup_user: TokenSessionOf<SessionPutProfile>,
    req: Json<PutProfileRequest>,
) -> actix_web::Result<HttpResponse, error::RegisterUsername> {
    validate_register_req(&req)?;

    let mut txn = db.begin().await?;

    let mut upsert_txn = txn.begin().await?;

    upsert_profile(&mut upsert_txn, &req, signup_user.claims.user_id).await?;

    db::session::delete(&mut upsert_txn, &signup_user.claims.token).await?;

    upsert_txn.commit().await?;

    let login_ttl = settings
        .login_token_valid_duration
        .unwrap_or(Duration::weeks(2));

    let session = db::session::create(
        &mut txn,
        signup_user.claims.user_id,
        Some(&(Utc::now() + login_ttl)),
        SessionMask::GENERAL,
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

/// Delete your account
#[api_v2_operation]
async fn delete(
    db: Data<PgPool>,
    session: TokenSessionOf<SessionDelete>,
) -> Result<NoContentClearAuth, error::Server> {
    sqlx::query!(
        r#"delete from "user" where id = $1"#,
        session.claims.user_id
    )
    .execute(db.as_ref())
    .await?;

    Ok(NoContentClearAuth)
}

/// Reset password
#[api_v2_operation]
async fn reset_password(
    config: Data<RuntimeSettings>,
    req: Json<<ResetPassword as ApiEndpoint>::Req>,
    db: Data<PgPool>,
    mail: ServiceData<mail::Client>,
) -> Result<NoContent, error::Service> {
    let req = req.into_inner();

    let mut txn = db.begin().await?;

    let user = sqlx::query!(
        "select user_id from user_email where email = $1::text",
        &req.email
    )
    .fetch_optional(&mut txn)
    .await?;

    let user_id = match user {
        Some(user) => user.user_id,
        None => return Ok(NoContent),
    };

    send_password_email(
        &mut txn,
        user_id,
        req.email,
        mail.as_ref(),
        &config.remote_target().pages_url(),
    )
    .await?;

    txn.commit().await?;

    Ok(NoContent)
}

/// Change password
#[api_v2_operation]
async fn put_password(
    db: Data<PgPool>,
    req: Json<<ChangePassword as ApiEndpoint>::Req>,
) -> Result<NoContent, error::ServiceSession> {
    let ChangePasswordRequest::Change {
        password,
        force_logout,
        token,
    } = req.into_inner();

    if password.is_empty() {
        return Err(anyhow::anyhow!("properly handle empty password error").into());
    }

    let mut txn = db.begin().await?;

    let user_id = db::session::get_onetime(&mut txn, SessionMask::CHANGE_PASSWORD, &token)
        .await?
        .ok_or(error::ServiceSession::Unauthorized)?;

    // todo: handle duplicate emails

    let email = sqlx::query!(
        r#"select email::text as "email!" from user_email where user_id = $1 for share"#,
        user_id
    )
    .fetch_optional(&mut txn)
    .await?;

    let email = match email {
        Some(email) => email.email,
        None => return Err(anyhow::anyhow!("Handle no confirmed email").into()),
    };

    let pass_hash = hash_password(password).await?;

    sqlx::query!("delete from user_auth_basic where user_id = $1", user_id)
        .execute(&mut txn)
        .await?;

    let user_to_delete = sqlx::query!(
        r#"select user_id from user_auth_basic where user_id <> $1 and email = $2 for update"#,
        user_id,
        email as _
    )
    .fetch_optional(&mut txn)
    .await?;

    if let Some(user_to_delete) = user_to_delete {
        sqlx::query!(
            r#"delete from "user" where id = $1"#,
            user_to_delete.user_id
        )
        .execute(&mut txn)
        .await?;
    }

    sqlx::query!(
        r#"
insert into user_auth_basic (user_id, email, password)
values ($1, $2::text, $3)
"#,
        user_id,
        &email,
        pass_hash.to_string(),
    )
    .execute(&mut txn)
    .await?;

    if force_logout {
        sqlx::query!("delete from session where user_id = $1", user_id)
            .execute(&mut txn)
            .await?;
    }

    txn.commit().await?;

    Ok(NoContent)
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(Profile::PATH, Profile::METHOD.route().to(get_profile))
        .route(Create::PATH, Create::METHOD.route().to(create_user))
        .route(
            VerifyEmail::PATH,
            VerifyEmail::METHOD.route().to(verify_email),
        )
        .route(
            ResetPassword::PATH,
            ResetPassword::METHOD.route().to(reset_password),
        )
        .route(
            ChangePassword::PATH,
            ChangePassword::METHOD.route().to(put_password),
        )
        .route(PutProfile::PATH, PutProfile::METHOD.route().to(put_profile))
        .route(UserLookup::PATH, UserLookup::METHOD.route().to(user_lookup))
        .route(Delete::PATH, Delete::METHOD.route().to(delete))
        .route(GetColors::PATH, GetColors::METHOD.route().to(color::get))
        .route(
            UpdateColor::PATH,
            UpdateColor::METHOD.route().to(color::update),
        )
        .route(
            CreateColor::PATH,
            CreateColor::METHOD.route().to(color::create),
        )
        .route(
            DeleteColor::PATH,
            DeleteColor::METHOD.route().to(color::delete),
        )
        .route(GetFonts::PATH, GetFonts::METHOD.route().to(font::get))
        .route(
            UpdateFont::PATH,
            UpdateFont::METHOD.route().to(font::update),
        )
        .route(
            CreateFont::PATH,
            CreateFont::METHOD.route().to(font::create),
        )
        .route(
            DeleteFont::PATH,
            DeleteFont::METHOD.route().to(font::delete),
        );
}
