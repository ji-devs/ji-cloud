use actix_web::{
    cookie::Cookie,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use chrono::{Duration, Utc};
use paperclip::actix::api_v2_operation;
use reqwest::Url;
use sqlx::{postgres::PgDatabaseError, PgPool};

use core::settings::{GoogleOAuth, RuntimeSettings};
use shared::{
    config::RemoteTarget,
    domain::session::{
        CreateSessionOAuthRequest, CreateSessionResponse, GetOAuthUrlResponse,
        GetOAuthUrlServiceKind, NewSessionResponse, OAuthUrlKind,
    },
};

use crate::{
    db, error,
    google_oauth::{self, oauth_url},
    jwk::{self, IdentityClaims},
    token::{create_auth_token, SessionMask},
};
use shared::domain::session::OAuthUserProfile;

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

#[api_v2_operation]
pub async fn get_url(
    req: HttpRequest,
    config: Data<RuntimeSettings>,
    Path((service_kind, url_kind)): Path<(GetOAuthUrlServiceKind, OAuthUrlKind)>,
) -> Result<Json<GetOAuthUrlResponse>, error::Service> {
    match service_kind {
        GetOAuthUrlServiceKind::Google => {}
        it => return Err(anyhow::anyhow!("Unsupported OAuth service kind: {:?}", it).into()),
    }

    let oauth_config = config
        .google_oauth
        .as_ref()
        .ok_or(error::Service::DisabledService(
            error::ServiceKind::GoogleOAuth,
        ))?;

    let route = oauth_url(config.remote_target(), url_kind);

    let mut url: Url = req.url_for_static("google_cloud_oauth")?;

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

#[api_v2_operation]
/// Login with OAuth
/// May return resources for *signing up* if the user doesn't exist.
pub async fn create(
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
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
) -> Result<(CreateSessionResponse, Cookie<'static>), error::OAuth> {
    let redirect_url = google_oauth::oauth_url(remote_target, redirect_kind);

    let tokens = google_oauth::convert_oauth_code(config, code, &redirect_url).await?;

    let claims: IdentityClaims = jwks.verify_oauth(&tokens.id_token, 3).await?;

    let profile = OAuthUserProfile {
        email: claims.email.clone(),
        name: claims.name,
        profile_picture: claims.profile_picture,
        given_name: claims.given_name,
        family_name: claims.family_name,
        locale: claims.locale,
    };

    let mut txn = db.begin().await?;

    let google_auth = sqlx::query!(
        "select user_id from user_auth_google where google_id = $1",
        &claims.google_id
    )
    .fetch_optional(&mut txn)
    .await?;

    let (user_id, mask) = match &google_auth {
        Some(google_auth) => {
            // make sure that the user either has a profile, or can only *create* one.
            let check_profile = sqlx::query!(
                r#"select exists(select 1 from user_profile where user_id = $1) as "exists!""#,
                google_auth.user_id
            )
            .fetch_one(&mut txn)
            .await?;

            let mask = if check_profile.exists {
                SessionMask::GENERAL
            } else {
                SessionMask::PUT_PROFILE | SessionMask::DELETE_ACCOUNT
            };

            (google_auth.user_id, mask)
        }
        None => {
            if !claims.email_verified {
                return Err(error::OAuth::Google(error::GoogleOAuth::UnverifiedEmail));
            }

            let id = sqlx::query!(r#"insert into "user" default values returning id"#)
                .fetch_one(&mut txn)
                .await?
                .id;

            sqlx::query!(
                r"insert into user_auth_google (user_id, google_id) values ($1, $2)",
                id,
                &claims.google_id
            )
            .execute(&mut txn)
            .await?;

            sqlx::query!(
                "insert into user_email (user_id, email) values ($1, $2::text)",
                id,
                &claims.email
            )
            .execute(&mut txn)
            .await
            .map_err(handle_user_email_error)?;
            (id, SessionMask::PUT_PROFILE)
        }
    };

    let login_ttl = login_token_valid_duration.unwrap_or(Duration::weeks(2));

    let valid_until = Utc::now()
        + if mask.contains(SessionMask::PUT_PROFILE) {
            Duration::hours(1)
        } else {
            login_ttl
        };

    let session = db::session::create(&mut txn, user_id, Some(&valid_until), mask, None).await?;

    txn.commit().await?;

    let (csrf, cookie) = create_auth_token(token_secret, local_insecure, login_ttl, &session)?;

    let response = NewSessionResponse { csrf };

    let response = if !mask.contains(SessionMask::GENERAL) {
        CreateSessionResponse::Register {
            response,
            oauth_profile: Some(profile),
        }
    } else {
        CreateSessionResponse::Login(response)
    };

    Ok((response, cookie))
}
