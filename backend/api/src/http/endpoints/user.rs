use std::sync::Arc;

use actix_web::{
    web::{Bytes, Data, Json, Query, ServiceConfig},
    HttpResponse,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{Duration, Utc};
use core::{config::IMAGE_BODY_SIZE_LIMIT, settings::RuntimeSettings};
use futures::try_join;
use rand::thread_rng;
use sendgrid::v3::Email;
use serde::{Deserialize, Serialize};
use shared::{
    api::endpoints::{
        user::{
            self, Browse, BrowsePlaylists, BrowseFollowers, BrowseFollowing, BrowsePublicUser,
            BrowseResources, BrowseUserJigs, ChangePassword, Create, CreateColor, CreateFont,
            CreateProfile, Delete, DeleteColor, DeleteFont, Follow, GetColors, GetFonts,
            GetPublicUser, PatchProfile, Profile, ResetEmail, ResetPassword, Search, SearchUser,
            Unfollow, UpdateColor, UpdateFont, UserLookup, VerifyEmail, VerifyResetEmail,
        },
        ApiEndpoint, PathParts,
    },
    domain::{
        image::{ImageId, ImageSize},
        session::{NewSessionResponse, OAuthProvider},
        user::{
            ChangePasswordRequest, CreateProfileRequest, ResetEmailResponse, UserBrowseResponse,
            UserId, UserLookupQuery, UserSearchResponse, VerifyEmailRequest,
            VerifyResetEmailRequest,
        },
    },
    media::MediaLibrary,
};
use sqlx::{postgres::PgDatabaseError, Acquire, PgConnection, PgPool};
use tracing::{instrument, Instrument};

use crate::{
    db::{self, user::upsert_profile},
    domain::NoContentClearAuth,
    error::{self, ServiceKind},
    extractor::{ScopeAdmin, SessionCreateProfile, SessionDelete, TokenSessionOf, TokenUser},
    service::{mail, s3, ServiceData},
    token::{create_auth_token, SessionMask},
};
use crate::{
    extractor::TokenUserWithScope,
    token::{create_update_email_token, validate_token},
};

use super::jig::page_limit;

mod color;
mod font;
pub mod public_user;

#[instrument(skip(txn, email_address, mail))]
async fn send_verification_email(
    txn: &mut PgConnection,
    user_id: UserId,
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

#[instrument(skip(txn, email_address, mail))]
async fn send_welcome_jigzi_email(
    txn: &mut PgConnection,
    user_id: UserId,
    email_address: String,
    mail: &mail::Client,
    pages_url: &str,
) -> Result<(), error::Service> {
    let first_name = db::user::get_given_name(&mut *txn, user_id).await?;

    let template = mail
        .welcome_jigzi_template()
        .map_err(error::Service::DisabledService)?;

    mail.send_welcome_jigzi(
        template,
        Email::new(email_address),
        pages_url.to_string(),
        first_name,
    )
    .await
    .map_err(|e| error::Service::InternalServerError(e))?;

    Ok(())
}

#[instrument(skip(txn, email_address, mail))]
async fn send_password_email(
    txn: &mut PgConnection,
    user_id: UserId,
    email_address: String,
    mail: &mail::Client,
    pages_url: &str,
    is_oauth: bool,
) -> Result<(), error::Service> {
    if !is_oauth {
        let session = db::session::create(
            &mut *txn,
            user_id,
            Some(&(Utc::now() + Duration::hours(1))),
            SessionMask::CHANGE_PASSWORD,
            None,
        )
        .await?;

        let first_name = db::user::get_given_name(&mut *txn, user_id).await?;

        let template = mail
            .password_reset_template()
            .map_err(error::Service::DisabledService)?;

        let email_link = format!("{}/user/password-reset/{}", pages_url, session);

        mail.send_password_reset(template, Email::new(email_address), email_link, first_name)
            .await?;
    } else {
        mail.send_oauth_password_reset(Email::new(email_address), OAuthProvider::Google)
            .await?;
    }

    Ok(())
}

#[instrument(skip(txn, mail))]
async fn send_reset_email(
    txn: &mut PgConnection,
    user_id: UserId,
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

    let first_name = db::user::get_given_name(&mut *txn, user_id).await?;

    let template = mail
        .email_reset_template()
        .map_err(error::Service::DisabledService)?;

    let email_link = format!("{}/user/verify-email-reset/{}", pages_url, session);

    // email_link contains the session id
    mail.send_email_reset(template, Email::new(email_address), email_link, first_name)
        .await?;

    Ok(())
}

#[instrument(skip_all)]
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
#[instrument(skip_all)]
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

    let email = req.email.to_lowercase();

    // FIXME simplify these queries
    let exists_basic = sqlx::query!(
        r#"select exists(select 1 from user_auth_basic where email = $1::text) as "exists!""#,
        email
    )
    .fetch_one(&mut txn)
    .instrument(tracing::info_span!("check basic exists"))
    .await?
    .exists;

    let exists_google = sqlx::query!(
        r#"select exists(select 1 from user_email where email = $1::text) as "exists!""#,
        email
    )
    .fetch_one(&mut txn)
    .instrument(tracing::info_span!("check google exists"))
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
        .instrument(tracing::info_span!("insert user"))
        .await?;

    let pass_hash = hash_password(req.password).await?;

    sqlx::query!(
        "insert into user_auth_basic (user_id, email, password) values ($1, $2::text, $3)",
        user.id,
        email,
        pass_hash.to_string(),
    )
    .execute(&mut txn)
    .instrument(tracing::info_span!("insert user_basic_auth"))
    .await?;

    send_verification_email(
        &mut txn,
        UserId(user.id),
        email,
        &mail,
        &config.remote_target().pages_url(),
    )
    .await
    .map_err(error::Register::from)?;

    txn.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Verify emails
#[instrument(skip_all)]
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

            let lowercase_email = &email.to_lowercase();

            // todo: make this more future proof and exhaustive.

            let user = sqlx::query!(
                r#"
select user_id          "id!: UserId"
from user_auth_basic
where
    email = $1::text and
    not exists(select 1 from user_email where email = $1)
"#,
                lowercase_email
            )
            .fetch_optional(&mut txn)
            .instrument(tracing::info_span!("get user_id"))
            .await?;

            let user = match user {
                Some(user) => user,
                None => return Ok(HttpResponse::NoContent().into()),
            };

            // make sure they can't use the old link anymore
            db::session::clear_any(&mut txn, user.id, SessionMask::VERIFY_EMAIL).await?;

            send_verification_email(
                &mut txn,
                user.id,
                lowercase_email.to_string(),
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
                error::Service::Forbidden => error::VerifyEmail::Forbidden,

                error::Service::ResourceNotFound => error::VerifyEmail::ResourceNotFound,
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
select session.user_id, lower(user_auth_basic.email)
from session
inner join user_auth_basic on user_auth_basic.user_id = session.user_id
where
    session.token = $1 and
    session.expires_at > now() and
    (session.scope_mask & $2) = $2
returning user_id as "id!: UserId"
"#,
                token,
                SessionMask::VERIFY_EMAIL.bits(),
            )
            .fetch_optional(&mut txn) // Result<Option<UserId>, Error>
            .instrument(tracing::info_span!("insert user_email"))
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
            db::session::clear_any(&mut txn, user.id, SessionMask::VERIFY_EMAIL).await?;

            let login_ttl = config
                .login_token_valid_duration
                .unwrap_or(Duration::weeks(2));

            let valid_until = Utc::now() + Duration::hours(1);

            let session = db::session::create(
                &mut txn,
                user.id,
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
#[instrument(skip(db))]
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
#[instrument(skip_all)]
async fn create_profile(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    s3: ServiceData<s3::Client>,
    signup_user: TokenSessionOf<SessionCreateProfile>,
    req: Json<CreateProfileRequest>,
    mail: ServiceData<mail::Client>,
) -> actix_web::Result<HttpResponse, error::UserUpdate> {
    validate_register_req(&req)?;

    let req = req.into_inner();

    let user_id = UserId(signup_user.claims.user_id);

    let profile_image_id: Option<ImageId> = match req.profile_image_url {
        Some(ref url) => create_user_profile_image(&db, &s3, &url, &user_id)
            .await
            .ok(),
        None => None,
    };

    let mut txn = db.begin().await?;

    let mut upsert_txn = txn.begin().await?;

    upsert_profile(&mut upsert_txn, &req, profile_image_id, user_id).await?;

    db::session::delete(&mut upsert_txn, &signup_user.claims.token).await?;

    upsert_txn.commit().await?;

    let login_ttl = settings
        .login_token_valid_duration
        .unwrap_or(Duration::weeks(2));

    let session = db::session::create(
        &mut txn,
        user_id,
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

    let email = db::user::get_email(&mut txn, user_id).await?;

    send_welcome_jigzi_email(
        &mut txn,
        user_id,
        email,
        &mail,
        &settings.remote_target().pages_url(),
    )
    .await
    .map_err(|e| {
        error::UserUpdate::InternalServerError(anyhow::anyhow!(
            "failed to send welcome jigzi email: {:?}",
            e
        ))
    })?;

    txn.commit().await?;

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(NewSessionResponse { csrf }))
}

#[instrument(skip(pool, s3))]
async fn create_user_profile_image(
    pool: &PgPool,
    s3: &s3::Client,
    url: &str,
    user_id: &UserId,
) -> anyhow::Result<ImageId> {
    // create entry in user library library -> ID
    let profile_image_id = db::image::user::create(&*pool, user_id, ImageSize::UserProfile).await?;

    let client: reqwest::Client = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // todo: this `?` should be a ClientError or "proxy/gateway error"
    let mut response: reqwest::Response = client
        .get(url)
        .send()
        .instrument(tracing::info_span!("fetch url"))
        .await?;

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
        crate::image_ops::generate_images(&original, ImageSize::Sticker)
    })
    .instrument(tracing::info_span!("process image"))
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
    .instrument(tracing::info_span!("update user_image_upload"))
    .await?;

    Ok(profile_image_id)
}

/// reset user email
#[instrument(skip(settings, mail, db, claims))]
async fn email_reset(
    settings: Data<RuntimeSettings>,
    mail: ServiceData<mail::Client>,
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<ResetEmail as ApiEndpoint>::Req>,
) -> Result<Json<<ResetEmail as ApiEndpoint>::Res>, error::Register> {
    // add authorized user to get user id
    let req = req.into_inner();
    let user_id = claims.user_id();

    let mut txn = db.begin().await?;

    let lowercase_email = req.email.to_lowercase();

    // 0. validate the email
    // FIXME simplify these queries - maybe make this a separate function to be used with update email
    let exists_basic = sqlx::query!(
        r#"select exists(select 1 from user_auth_basic where email = $1::text) as "exists!""#,
        lowercase_email
    )
    .fetch_one(&mut txn)
    .instrument(tracing::info_span!("validate email"))
    .await?
    .exists;

    let exists_google = sqlx::query!(
        r#"select exists(select 1 from user_email where email = $1::text) as "exists!""#,
        lowercase_email
    )
    .fetch_one(&mut txn)
    .instrument(tracing::info_span!("check google exists"))
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
    let paseto_token: String = create_update_email_token(
        &settings.token_secret,
        Duration::hours(1),
        &lowercase_email,
        Utc::now(),
        &user_id.0,
    )?;

    // 2. Send email reset email with token
    send_reset_email(
        &mut txn,
        user_id,
        lowercase_email,
        &mail,
        &settings.remote_target().pages_url(),
    )
    .await
    .map_err(error::Register::from)?;

    txn.commit().await?;

    Ok(Json(ResetEmailResponse { paseto_token }))
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

impl From<TokenUser> for UserId {
    fn from(token: TokenUser) -> Self {
        Self(token.0.user_id)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailToken {
    /// The new email instance this token is for.
    pub email: String,

    /// The uuid instance this token is for.
    pub user_id: UserId,
}

/// Verify email reset email
#[instrument(skip(settings, mail, db))]
async fn verify_email_reset(
    settings: Data<RuntimeSettings>,
    mail: ServiceData<mail::Client>,
    db: Data<PgPool>,
    req: Json<<VerifyResetEmail as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::VerifyEmail> {
    let req = req.into_inner();

    match req {
        VerifyResetEmailRequest::Resend { paseto_token } => {
            let mut txn = db.begin().await?;

            let token: EmailToken = validate_email_token(paseto_token, &settings.token_secret)?;

            let email = &token.email.to_lowercase();

            // 1. generate a paseto token for this instance
            let paseto_token: String = create_update_email_token(
                &settings.token_secret,
                Duration::hours(1),
                &email,
                Utc::now(),
                &token.user_id.into(),
            )?;

            // make sure they can't use the old link anymore
            db::session::clear_any(&mut txn, token.user_id, SessionMask::CHANGE_EMAIL).await?;

            send_reset_email(
                &mut txn,
                token.user_id,
                email.to_string(),
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
                error::Service::Forbidden => error::VerifyEmail::Forbidden,

                error::Service::ResourceNotFound => error::VerifyEmail::Forbidden,
            })?;

            txn.commit().await?;

            Ok(HttpResponse::Ok().json(ResetEmailResponse { paseto_token }))
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
                token.user_id.0
            )
            .fetch_optional(&mut txn)
            .instrument(tracing::info_span!("email exists"))
            .await?;

            let email = match email {
                Some(email) => email.email.to_lowercase(),
                None => return Err(anyhow::anyhow!("Handle no confirmed email").into()),
            };

            // make sure they can't use the link, now that they're email has changed.
            db::session::clear_any(&mut txn, token.user_id, SessionMask::CHANGE_EMAIL).await?;

            // update user_auth_basic table
            sqlx::query!(
                r#"
        update user_auth_basic
        set email = $3::text
        where user_id = $1 and email = $2::text
        "#,
                token.user_id.0,
                &email,
                token.email
            )
            .execute(&mut txn)
            .instrument(tracing::info_span!("update user_auth_basic"))
            .await?;

            // update user_email table
            sqlx::query!(
                r#"
        update user_email
        set email = $3::text
        where user_id = $1 and email = $2::text
        "#,
                token.user_id.0,
                &email,
                token.email
            )
            .execute(&mut txn)
            .instrument(tracing::info_span!("update user_email"))
            .await?;

            if force_logout {
                sqlx::query!("delete from session where user_id = $1", token.user_id.0)
                    .execute(&mut txn)
                    .instrument(tracing::info_span!("force logout"))
                    .await?;
            }

            txn.commit().await?;

            Ok(HttpResponse::NoContent().finish())
        }
    }
}

#[instrument(skip_all)]
pub fn validate_email_token(
    paseto_token: String,
    token_key: &[u8; 32],
) -> Result<EmailToken, error::VerifyEmail> {
    let token = validate_token(&paseto_token, None, token_key)
        .map_err(|_| error::VerifyEmail::Email(error::Email::Empty))?;

    let (user_id, new_email) = (
        serde_json::from_value::<UserId>(token["id"].clone())?,
        serde_json::from_value::<String>(token["sub"].clone())?,
    );

    Ok(EmailToken {
        email: new_email.to_lowercase(),
        user_id,
    })
}

/// Update your profile.
#[instrument(skip(db, claims))]
async fn patch_profile(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<PatchProfile as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::UserUpdate> {
    validate_patch_profile_req(&req)?;
    let user_id = claims.user_id();

    db::user::update_profile(&*db, user_id, req.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Get a user's profile.
#[instrument(skip_all)]
async fn get_profile(
    db: Data<PgPool>,
    claims: TokenUser,
) -> Result<Json<<Profile as ApiEndpoint>::Res>, error::UserNotFound> {
    // todo: figure out how to do `<Profile as ApiEndpoint>::Err`
    let user_id = claims.user_id();

    match db::user::get_profile(db.as_ref(), &user_id).await? {
        Some(mut profile) => {
            let account_summary =
                db::account::get_user_account_summary(db.as_ref(), &user_id).await?;
            profile.account_summary = account_summary;
            Ok(Json(profile))
        }
        None => Err(error::UserNotFound::UserNotFound),
    }
}

/// Delete your account
#[instrument(skip_all)]
async fn delete(
    db: Data<PgPool>,
    session: TokenSessionOf<SessionDelete>,
    algolia: ServiceData<crate::algolia::Manager>,
) -> Result<NoContentClearAuth, error::Server> {
    sqlx::query!(
        r#"delete from "user" where id = $1"#,
        session.claims.user_id
    )
    .execute(db.as_ref())
    .await?;

    algolia.delete_public_user(session.claims.user_id).await;

    Ok(NoContentClearAuth)
}

/// Reset password
#[instrument(skip_all)]
async fn reset_password(
    config: Data<RuntimeSettings>,
    req: Json<<ResetPassword as ApiEndpoint>::Req>,
    db: Data<PgPool>,
    mail: ServiceData<mail::Client>,
) -> Result<HttpResponse, error::Service> {
    let req = req.into_inner();

    let mut txn = db.begin().await?;

    let email = req.email.to_lowercase();

    // includes check for oauth email
    let user = sqlx::query!(
        r#"
        select user_id "user_id: UserId",
        (
           select
             case
                when exists(select 1 from user_auth_basic where user_auth_basic.email = lower($1::text)) = true then false
                else true
            end
        )     as "is_oauth!"
         from user_email
         where email = lower($1::text)"#,
        &email
    )
    .fetch_optional(&mut txn)
    .instrument(tracing::info_span!("get user_id"))
    .await?;

    let (user_id, is_oauth) = match user {
        Some(user) => (user.user_id, user.is_oauth),
        None => return Ok(HttpResponse::NoContent().finish()),
    };

    send_password_email(
        &mut txn,
        user_id,
        email,
        mail.as_ref(),
        &config.remote_target().pages_url(),
        is_oauth,
    )
    .await?;

    txn.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Change password
#[instrument(skip_all)]
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
        user_id.0
    )
    .fetch_optional(&mut txn)
    .instrument(tracing::info_span!("get email address"))
    .await?;

    let email = match email {
        Some(email) => email.email.to_lowercase(),
        None => return Err(anyhow::anyhow!("Handle no confirmed email").into()),
    };

    let pass_hash = hash_password(password).await?;

    sqlx::query!("delete from user_auth_basic where user_id = $1", user_id.0)
        .execute(&mut txn)
        .instrument(tracing::info_span!("delete user_auth_basic"))
        .await?;

    let user_to_delete = sqlx::query!(
        r#"select user_id "id!: UserId" from user_auth_basic where user_id <> $1 and email = $2 for update"#,
        user_id.0,
        email as _
    )
    .fetch_optional(&mut txn)
    .instrument(tracing::info_span!("get user_id"))
    .await?;

    if let Some(user_to_delete) = user_to_delete {
        sqlx::query!(r#"delete from "user" where id = $1"#, user_to_delete.id.0)
            .execute(&mut txn)
            .instrument(tracing::info_span!("delete user_to_delete"))
            .await?;
    }

    sqlx::query!(
        r#"
insert into user_auth_basic (user_id, email, password)
values ($1, $2::text, $3)
"#,
        user_id.0,
        &email,
        pass_hash.to_string(),
    )
    .execute(&mut txn)
    .instrument(tracing::info_span!("insert user_auth_basic"))
    .await?;

    if force_logout {
        sqlx::query!("delete from session where user_id = $1", user_id.0)
            .execute(&mut txn)
            .instrument(tracing::info_span!("delete session"))
            .await?;
    }

    txn.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

#[instrument(skip_all)]
async fn browse(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    query: Option<Query<<user::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<user::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Auth::InternalServerError(e))?;

    let browse_future = db::user::browse(
        db.as_ref(),
        query.user_id,
        query.page.unwrap_or(0) as i32,
        page_limit,
    );

    let total_count_future = db::user::filtered_count(db.as_ref(), query.user_id);

    let (users, total_count) = try_join!(browse_future, total_count_future,)?;

    let pages = (total_count / (page_limit as u64)
        + (total_count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(UserBrowseResponse {
        users,
        pages,
        total_user_count: total_count,
    }))
}

/// Search for public user profile.
pub async fn search(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    algolia: ServiceData<crate::algolia::Client>,
    query: Option<Query<<user::SearchUser as ApiEndpoint>::Req>>,
) -> Result<Json<<user::SearchUser as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Service::InternalServerError(e))?;

    let (ids, pages, total_hits) = algolia
        .search_user(&query.q, query.user_id, page_limit, query.page)
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let users: Vec<_> = db::user::get_by_ids(db.as_ref(), &ids).await?;

    Ok(Json(UserSearchResponse {
        users,
        pages,
        total_user_count: total_hits,
    }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <ResetEmail as ApiEndpoint>::Path::PATH,
        ResetEmail::METHOD.route().to(email_reset),
    )
    .route(
        <VerifyResetEmail as ApiEndpoint>::Path::PATH,
        VerifyResetEmail::METHOD.route().to(verify_email_reset),
    )
    .route(
        <Profile as ApiEndpoint>::Path::PATH,
        Profile::METHOD.route().to(get_profile),
    )
    .route(
        <Create as ApiEndpoint>::Path::PATH,
        Create::METHOD.route().to(create_user),
    )
    .route(
        <Browse as ApiEndpoint>::Path::PATH,
        Browse::METHOD.route().to(browse),
    )
    .route(
        <SearchUser as ApiEndpoint>::Path::PATH,
        SearchUser::METHOD.route().to(search),
    )
    .route(
        <VerifyEmail as ApiEndpoint>::Path::PATH,
        VerifyEmail::METHOD.route().to(verify_email),
    )
    .route(
        <ResetPassword as ApiEndpoint>::Path::PATH,
        ResetPassword::METHOD.route().to(reset_password),
    )
    .route(
        <ChangePassword as ApiEndpoint>::Path::PATH,
        ChangePassword::METHOD.route().to(put_password),
    )
    .route(
        <CreateProfile as ApiEndpoint>::Path::PATH,
        CreateProfile::METHOD.route().to(create_profile),
    )
    .route(
        <PatchProfile as ApiEndpoint>::Path::PATH,
        PatchProfile::METHOD.route().to(patch_profile),
    )
    .route(
        <UserLookup as ApiEndpoint>::Path::PATH,
        UserLookup::METHOD.route().to(user_lookup),
    )
    .route(
        <Delete as ApiEndpoint>::Path::PATH,
        Delete::METHOD.route().to(delete),
    )
    .route(
        <GetColors as ApiEndpoint>::Path::PATH,
        GetColors::METHOD.route().to(color::get),
    )
    .route(
        <UpdateColor as ApiEndpoint>::Path::PATH,
        UpdateColor::METHOD.route().to(color::update),
    )
    .route(
        <CreateColor as ApiEndpoint>::Path::PATH,
        CreateColor::METHOD.route().to(color::create),
    )
    .route(
        <DeleteColor as ApiEndpoint>::Path::PATH,
        DeleteColor::METHOD.route().to(color::delete),
    )
    .route(
        <GetFonts as ApiEndpoint>::Path::PATH,
        GetFonts::METHOD.route().to(font::get),
    )
    .route(
        <UpdateFont as ApiEndpoint>::Path::PATH,
        UpdateFont::METHOD.route().to(font::update),
    )
    .route(
        <CreateFont as ApiEndpoint>::Path::PATH,
        CreateFont::METHOD.route().to(font::create),
    )
    .route(
        <DeleteFont as ApiEndpoint>::Path::PATH,
        DeleteFont::METHOD.route().to(font::delete),
    )
    .route(
        <Search as ApiEndpoint>::Path::PATH,
        Search::METHOD.route().to(public_user::search),
    )
    .route(
        <BrowsePublicUser as ApiEndpoint>::Path::PATH,
        BrowsePublicUser::METHOD.route().to(public_user::browse),
    )
    .route(
        <BrowseUserJigs as ApiEndpoint>::Path::PATH,
        BrowseUserJigs::METHOD
            .route()
            .to(public_user::browse_user_jigs),
    )
    .route(
        <BrowseResources as ApiEndpoint>::Path::PATH,
        BrowseResources::METHOD
            .route()
            .to(public_user::browse_user_resources),
    )
    .route(
        <BrowsePlaylists as ApiEndpoint>::Path::PATH,
        BrowsePlaylists::METHOD
            .route()
            .to(public_user::browse_user_playlists),
    )
    .route(
        <GetPublicUser as ApiEndpoint>::Path::PATH,
        GetPublicUser::METHOD.route().to(public_user::get),
    )
    .route(
        <BrowseFollowers as ApiEndpoint>::Path::PATH,
        BrowseFollowers::METHOD
            .route()
            .to(public_user::browse_user_followers),
    )
    .route(
        <BrowseFollowing as ApiEndpoint>::Path::PATH,
        BrowseFollowing::METHOD
            .route()
            .to(public_user::browse_user_followings),
    )
    .route(
        <Follow as ApiEndpoint>::Path::PATH,
        Follow::METHOD.route().to(public_user::follow),
    )
    .route(
        <Unfollow as ApiEndpoint>::Path::PATH,
        Unfollow::METHOD.route().to(public_user::unfollow),
    );
}
