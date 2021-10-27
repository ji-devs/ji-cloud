use std::sync::Arc;

use actix_web::{
    web::{Bytes, Data, Json, Query, ServiceConfig},
    HttpResponse,
};
use anyhow::anyhow;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{Duration, Utc};
use core::{config::IMAGE_BODY_SIZE_LIMIT, settings::RuntimeSettings};
use rand::thread_rng;
use sendgrid::v3::Email;
use serde::{Deserialize, Serialize};
use shared::domain::user::{ResetEmailResponse, VerifyResetEmailRequest};
use shared::{
    api::endpoints::{
        user::{
            ChangePassword, Create, CreateColor, CreateFont, CreateProfile, Delete, DeleteColor,
            DeleteFont, GetColors, GetFonts, PatchProfile, Profile, ResetEmail, ResetPassword,
            UpdateColor, UpdateFont, UserLookup, VerifyEmail, VerifyResetEmail,
        },
        ApiEndpoint,
    },
    domain::{
        image::{ImageId, ImageKind},
        session::NewSessionResponse,
        user::{ChangePasswordRequest, CreateProfileRequest, UserLookupQuery, VerifyEmailRequest},
    },
    media::MediaLibrary,
};
use sqlx::{postgres::PgDatabaseError, Acquire, PgConnection, PgPool};
use uuid::Uuid;

use crate::token::{create_update_email_token, validate_token};
use crate::{
    db::{self, user::upsert_profile},
    domain::NoContentClearAuth,
    error,
    extractor::{SessionCreateProfile, SessionDelete, TokenSessionOf, TokenUser},
    service::{mail, s3, ServiceData},
    token::{create_auth_token, SessionMask},
};

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

    let email_link = format!("{}/user/password-reset/{}", pages_url, session);

    mail.send_password_reset(template, Email::new(email_address), email_link)
        .await?;

    Ok(())
}

async fn send_reset_email(
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
        SessionMask::CHANGE_EMAIL,
        None,
    )
    .await?;

    let template = mail
        .email_reset_template()
        .map_err(error::Service::DisabledService)?;

    let email_link = format!("{}/user/verify-email-reset/{}", pages_url, session);

    // email_link contains the session id
    mail.send_email_reset(template, Email::new(email_address), email_link)
        .await?;

    Ok(())
}

async fn hash_password(pass: String) -> anyhow::Result<String> {
    let pass_hash = actix_web::web::block(move || {
        let password_hasher = Argon2::default();

        let salt = SaltString::generate(thread_rng());
        password_hasher
            .hash_password(pass.as_bytes(), &salt.as_salt())
            .map_err(|it| anyhow::anyhow!("{}", it))
            .map(|it| it.to_string())
    })
    .await??;

    Ok(pass_hash)
}

/// Create a user
async fn create_user(
    config: Data<RuntimeSettings>,
    mail: ServiceData<mail::Client>,
    db: Data<PgPool>,
    req: Json<<Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Register> {
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
            return Err(error::Email::TakenBasic.into());
        }
        (false, true) => {
            txn.rollback().await?;
            return Err(error::Email::TakenGoogle.into());
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

    Ok(HttpResponse::NoContent().finish())
}

/// Verify emails
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
                    error::VerifyEmail::Email(error::Email::TakenBasic)
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

fn validate_register_req(req: &CreateProfileRequest) -> Result<(), error::UserUpdate> {
    if req.username.is_empty() {
        return Err(error::UserUpdate::Username(error::Username::Empty));
    }

    Ok(())
}

/// Create a user profile.
async fn create_profile(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    s3: ServiceData<s3::Client>,
    signup_user: TokenSessionOf<SessionCreateProfile>,
    req: Json<CreateProfileRequest>,
) -> actix_web::Result<HttpResponse, error::UserUpdate> {
    validate_register_req(&req)?;

    let req = req.into_inner();

    let profile_image_id: Option<ImageId> = match req.profile_image_url {
        Some(ref url) => create_user_profile_image(&db, &s3, &url, &signup_user.claims.user_id)
            .await
            .ok(),
        None => None,
    };

    let mut txn = db.begin().await?;

    let mut upsert_txn = txn.begin().await?;

    upsert_profile(
        &mut upsert_txn,
        &req,
        profile_image_id,
        signup_user.claims.user_id,
    )
    .await?;

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

async fn create_user_profile_image(
    pool: &PgPool,
    s3: &s3::Client,
    url: &str,
    user_id: &Uuid,
) -> anyhow::Result<ImageId> {
    // create entry in user library library -> ID
    let profile_image_id = db::image::user::create(&*pool, user_id, ImageKind::UserProfile).await?;

    let client: reqwest::Client = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // todo: this `?` should be a ClientError or "proxy/gateway error"
    let mut response: reqwest::Response = client.get(url).send().await?;

    let mut data = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        let chunk: Bytes = chunk;
        if data.len() + chunk.len() < IMAGE_BODY_SIZE_LIMIT {
            data.extend_from_slice(&chunk[..]);
        } else {
            return Err(anyhow::anyhow!("todo: better error here (data too big)").into());
        }
    }

    log::trace!("data was {} bytes long", data.len());

    let data = Arc::new(data);

    // process
    let (original, resized, thumbnail) = actix_web::web::block(move || {
        let original = image::load_from_memory(&data)?;
        crate::image_ops::generate_images(&original, ImageKind::Sticker)
    })
    .await??;

    // upload to ID
    s3.upload_png_images(
        MediaLibrary::User,
        profile_image_id.0,
        original,
        resized,
        thumbnail,
    )
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
update user_image_upload
set uploaded_at       = now(),
    processed_at      = now(),
    processing_result = true
where image_id = $1
"#,
        profile_image_id.0
    )
    .execute(pool)
    .await?;

    Ok(profile_image_id)
}

/// reset user email
async fn email_reset(
    settings: Data<RuntimeSettings>,
    mail: ServiceData<mail::Client>,
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<ResetEmail as ApiEndpoint>::Req>,
) -> Result<Json<<ResetEmail as ApiEndpoint>::Res>, error::Register> {
    // add authorized user to get user id
    let req = req.into_inner();

    let mut txn = db.begin().await?;

    // 0. validate the email
    // FIXME simplify these queries - maybe make this a separate function to be used with update email
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
            return Err(error::Email::TakenBasic.into());
        }
        (false, true) => {
            txn.rollback().await?;
            return Err(error::Email::TakenGoogle.into());
        }
        (false, false) => (), // do nothing
    }

    // 1. generate a paseto token for this instance
    let token: String = create_update_email_token(
        &settings.token_secret,
        Duration::hours(1),
        &req.email,
        Utc::now(),
        &claims.0.user_id,
    )?;

    // 2. Send email reset email with token
    send_reset_email(
        &mut txn,
        claims.0.user_id,
        req.email,
        &mail,
        &settings.remote_target().pages_url(),
    )
    .await
    .map_err(error::Register::from)?;

    txn.commit().await?;

    Ok(Json(ResetEmailResponse { token }))
}

fn validate_patch_profile_req(
    req: &Json<<PatchProfile as ApiEndpoint>::Req>,
) -> Result<(), error::UserUpdate> {
    match &req.username {
        Some(username) if username.is_empty() => {
            Err(error::UserUpdate::Username(error::Username::Empty))
        }
        _ => Ok(()),
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailToken {
    /// The new email instance this token is for.
    pub email: String,

    /// The uuid instance this token is for.
    pub user_id: Uuid,
}

/// Verify email reset email
async fn verify_email_reset(
    settings: Data<RuntimeSettings>,
    mail: ServiceData<mail::Client>,
    db: Data<PgPool>,
    req: Json<<VerifyResetEmail as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::VerifyEmail> {
    let req = req.into_inner();

    match req {
        VerifyResetEmailRequest::Resend {
            paseto_token,
            email,
        } => {
            let mut txn = db.begin().await?;

            let token: EmailToken = validate_email_token(paseto_token, &settings.token_secret)?;

            // make sure they can't use the old link anymore
            db::session::clear_any(&mut txn, token.user_id, SessionMask::CHANGE_EMAIL).await?;

            send_reset_email(
                &mut txn,
                token.user_id,
                email,
                &mail,
                &settings.remote_target().pages_url(),
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

        VerifyResetEmailRequest::Verify {
            paseto_token,
            force_logout,
        } => {
            let mut txn = db.begin().await?;

            let token: EmailToken = validate_email_token(paseto_token, &settings.token_secret)?;

            // Check if email exists
            let email = sqlx::query!(
                r#"select email::text as "email!" from user_email where user_id = $1 for share"#,
                token.user_id
            )
            .fetch_optional(&mut txn)
            .await?;

            let email = match email {
                Some(email) => email.email,
                None => return Err(anyhow::anyhow!("Handle no confirmed email").into()),
            };

            // make sure they can't use the link, now that they're email has changed.
            db::session::clear_any(&mut txn, token.user_id, SessionMask::CHANGE_EMAIL).await?;

            sqlx::query!(
                r#"
        update user_auth_basic 
        set email = $3::text
        where user_id = $1 and email = $2::text
        "#,
                token.user_id,
                &email,
                token.email
            )
            .execute(&mut txn)
            .await?;

            sqlx::query!(
                r#"
        update user_email 
        set email = $3::text
        where user_id = $1 and email = $2::text
        "#,
                token.user_id,
                &email,
                token.email
            )
            .execute(&mut txn)
            .await?;

            if force_logout {
                sqlx::query!("delete from session where user_id = $1", &token.user_id)
                    .execute(&mut txn)
                    .await?;
            }

            txn.commit().await?;

            Ok(HttpResponse::NoContent().finish())
        }
    }
}

pub fn validate_email_token(
    paseto_token: String,
    token_key: &[u8; 32],
) -> Result<EmailToken, error::VerifyEmail> {
    let token = validate_token(&paseto_token, None, token_key)
        .map_err(|_| error::VerifyEmail::Email(error::Email::Empty))?;

    let (user_id, new_email) = (
        serde_json::from_value::<Uuid>(token["id"].clone())?,
        token["sub"].to_string(),
    );

    Ok(EmailToken {
        email: new_email,
        user_id,
    })
}

/// Update your profile.
async fn patch_profile(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<PatchProfile as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::UserUpdate> {
    validate_patch_profile_req(&req)?;

    db::user::update_profile(&*db, claims.0.user_id, req.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Get a user's profile.
async fn get_profile(
    db: Data<PgPool>,
    claims: TokenUser,
) -> Result<Json<<Profile as ApiEndpoint>::Res>, error::UserNotFound> {
    // todo: figure out how to do `<Profile as ApiEndpoint>::Err`

    db::user::get_profile(db.as_ref(), claims.0.user_id)
        .await?
        .map(Json)
        .ok_or(error::UserNotFound::UserNotFound)
}

/// Delete your account
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
async fn reset_password(
    config: Data<RuntimeSettings>,
    req: Json<<ResetPassword as ApiEndpoint>::Req>,
    db: Data<PgPool>,
    mail: ServiceData<mail::Client>,
) -> Result<HttpResponse, error::Service> {
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
        None => return Ok(HttpResponse::NoContent().finish()),
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

    Ok(HttpResponse::NoContent().finish())
}

/// Change password
async fn put_password(
    db: Data<PgPool>,
    req: Json<<ChangePassword as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::ServiceSession> {
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

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(ResetEmail::PATH, ResetEmail::METHOD.route().to(email_reset))
        .route(
            VerifyResetEmail::PATH,
            VerifyResetEmail::METHOD.route().to(verify_email_reset),
        )
        .route(Profile::PATH, Profile::METHOD.route().to(get_profile))
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
        .route(
            CreateProfile::PATH,
            CreateProfile::METHOD.route().to(create_profile),
        )
        .route(
            PatchProfile::PATH,
            PatchProfile::METHOD.route().to(patch_profile),
        )
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
