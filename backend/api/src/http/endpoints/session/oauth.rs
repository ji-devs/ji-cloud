use actix_web::{
    cookie::Cookie,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use chrono::{Duration, Utc};
use reqwest::Url;
use sqlx::{postgres::PgDatabaseError, PgPool};

use ji_core::settings::{GoogleOAuth, RuntimeSettings};
use shared::{
    config::RemoteTarget,
    domain::{
        session::{
            CreateSessionOAuthRequest, CreateSessionResponse, GetOAuthUrlResponse,
            GetOAuthUrlServiceKind, NewSessionResponse, OAuthUrlKind,
        },
        user::UserId,
    },
};

use crate::{
    db, error,
    google_oauth::{self, oauth_url},
    http::endpoints::user::send_verification_email,
    jwk::{self, IdentityClaims},
    service::{mail, ServiceData},
    token::{create_auth_token, SessionMask},
};
use shared::domain::session::OAuthUserProfile;
use shared::error::{IntoAnyhow, ServiceError, ServiceKindError};

fn handle_user_email_error(e: sqlx::Error) -> error::OAuth {
    let db_err = match &e {
        sqlx::Error::Database(e) => e.downcast_ref::<PgDatabaseError>(),
        _ => return e.into(),
    };

    match db_err.constraint() {
        Some("user_email_email_key") => error::OAuth::Conflict,

        _ => e.into(),
    }
}

pub async fn get_url(
    req: HttpRequest,
    config: Data<RuntimeSettings>,
    path: Path<(GetOAuthUrlServiceKind, OAuthUrlKind)>,
) -> Result<Json<GetOAuthUrlResponse>, ServiceError> {
    let (service_kind, url_kind) = path.into_inner();

    match service_kind {
        GetOAuthUrlServiceKind::Google => {}
        it => return Err(anyhow::anyhow!("Unsupported OAuth service kind: {:?}", it).into()),
    }

    let oauth_config = config
        .google_oauth
        .as_ref()
        .ok_or(ServiceError::DisabledService(ServiceKindError::GoogleOAuth))?;

    let route = oauth_url(config.remote_target(), url_kind);

    let mut url: Url = req.url_for_static("google_cloud_oauth").into_anyhow()?;

    // todo: add / verify `state`

    let url = url
        .query_pairs_mut()
        .append_pair("client_id", &oauth_config.client)
        .append_pair("response_type", "code")
        .append_pair("include_granted_scopes", "true")
        .append_pair("redirect_uri", &route)
        .append_pair("scope", "openid email")
        .finish()
        .to_string();

    Ok(Json(GetOAuthUrlResponse { url }))
}

/// Login with OAuth
/// May return resources for *signing up* if the user doesn't exist.
pub async fn create(
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    mail: ServiceData<mail::Client>,
    req: Json<CreateSessionOAuthRequest>,
    jwks: Data<jwk::JwkVerifier>,
) -> Result<HttpResponse, error::OAuth> {
    let (response, cookie) = match req.into_inner() {
        CreateSessionOAuthRequest::Google {
            code,
            redirect_kind,
        } => {
            let config = settings
                .google_oauth
                .as_ref()
                .ok_or(error::GoogleOAuth::Disabled)?;

            handle_google_oauth(
                &db,
                &config,
                &settings.token_secret,
                settings.is_local(),
                &jwks,
                &code,
                settings.login_token_valid_duration,
                settings.remote_target(),
                redirect_kind,
                &mail,
            )
            .await?
        }

        other => return Err(anyhow::anyhow!("Unsupported OAuth request kind: {:?}", other).into()),
    };

    Ok(HttpResponse::Created().cookie(cookie).json(response))
}

async fn handle_google_oauth(
    db: &PgPool,
    config: &GoogleOAuth,
    token_secret: &[u8; 32],
    local_insecure: bool,
    jwks: &jwk::JwkVerifier,
    code: &str,
    login_token_valid_duration: Option<Duration>,
    remote_target: RemoteTarget,
    redirect_kind: OAuthUrlKind,
    mail: &mail::Client,
) -> Result<(CreateSessionResponse, Cookie<'static>), error::OAuth> {
    let redirect_url = google_oauth::oauth_url(remote_target, redirect_kind);

    let tokens = google_oauth::convert_oauth_code(config, code, &redirect_url).await?;

    let claims: IdentityClaims = jwks.verify_google_user_oauth(&tokens.id_token, 3).await?;

    let mut txn = db.begin().await?;

    let google_auth = sqlx::query!(
        "select user_id from user_auth_google where google_id = $1",
        &claims.google_id
    )
    .fetch_optional(&mut txn)
    .await?;

    let (user_id, mask) = match &google_auth {
        Some(google_auth) => {
            // Check user status: blocked, has verified email, has profile
            let check_status = sqlx::query!(
                r#"select
                    exists(select 1 from user_profile where user_id = $1) as "has_profile!",
                    exists(select 1 from user_email where user_id = $1) as "has_verified_email!",
                    (select blocked from "user" where id = $1) as "blocked?"
                "#,
                google_auth.user_id
            )
            .fetch_one(&mut txn)
            .await?;

            if check_status.blocked.unwrap_or(false) {
                return Err(error::OAuth::Unauthorized);
            }

            let mask = if check_status.has_profile {
                SessionMask::GENERAL
            } else if check_status.has_verified_email {
                SessionMask::PUT_PROFILE | SessionMask::DELETE_ACCOUNT
            } else {
                // Email not yet verified - resend verification email
                let email = sqlx::query_scalar!(
                    r#"select unverified_email::text as "email!" from user_auth_google where user_id = $1 and unverified_email is not null"#,
                    google_auth.user_id
                )
                .fetch_one(&mut txn)
                .await
                .map_err(|_| anyhow::anyhow!("Google auth record missing email"))?;

                send_verification_email(
                    &mut txn,
                    UserId(google_auth.user_id),
                    email,
                    mail,
                    &remote_target.pages_url(),
                )
                .await
                .map_err(|e| error::OAuth::InternalServerError(e.into()))?;

                SessionMask::VERIFY_EMAIL
            };

            (google_auth.user_id, mask)
        }
        None => {
            if !claims.email_verified {
                return Err(error::OAuth::Google(error::GoogleOAuth::UnverifiedEmail));
            }

            // Check if email already exists (verified by another user)
            let email_exists = sqlx::query!(
                r#"select exists(select 1 from user_email where email = lower($1::text)) as "exists!""#,
                &claims.email
            )
            .fetch_one(&mut txn)
            .await?
            .exists;

            if email_exists {
                return Err(error::OAuth::Conflict);
            }

            let id = sqlx::query!(r#"insert into "user" default values returning id"#)
                .fetch_one(&mut txn)
                .await?
                .id;

            // Store email in user_auth_google (unverified) instead of user_email
            sqlx::query!(
                r"insert into user_auth_google (user_id, google_id, unverified_email) values ($1, $2, lower($3::text))",
                id,
                &claims.google_id,
                &claims.email
            )
            .execute(&mut txn)
            .await?;

            // Send verification email
            send_verification_email(
                &mut txn,
                UserId(id),
                claims.email.to_lowercase(),
                mail,
                &remote_target.pages_url(),
            )
            .await
            .map_err(|e| error::OAuth::InternalServerError(e.into()))?;

            (id, SessionMask::VERIFY_EMAIL)
        }
    };

    let login_ttl = login_token_valid_duration.unwrap_or(Duration::weeks(2));

    let valid_until = Utc::now()
        + if mask.contains(SessionMask::PUT_PROFILE) || mask.contains(SessionMask::VERIFY_EMAIL) {
            Duration::hours(1)
        } else {
            login_ttl
        };

    let session =
        db::session::create(&mut txn, UserId(user_id), Some(&valid_until), mask, None).await?;

    txn.commit().await?;

    let (csrf, cookie) = create_auth_token(token_secret, local_insecure, login_ttl, &session)?;

    let response = NewSessionResponse { csrf };

    let response = if !mask.contains(SessionMask::GENERAL) {
        let profile = OAuthUserProfile {
            email: claims.email.clone(),
            name: claims.name,
            profile_picture: claims.profile_picture,
            given_name: claims.given_name,
            family_name: claims.family_name,
            locale: claims.locale,
        };

        CreateSessionResponse::Register {
            response,
            oauth_profile: Some(profile),
            needs_email_verification: mask.contains(SessionMask::VERIFY_EMAIL),
        }
    } else {
        CreateSessionResponse::Login(response)
    };

    Ok((response, cookie))
}
