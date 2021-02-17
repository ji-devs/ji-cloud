use actix_web::{
    cookie::Cookie,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use chrono::Duration;
use config::RemoteTarget;
use core::settings::{GoogleOAuth, RuntimeSettings};
use paperclip::actix::{api_v2_operation, web::ServiceConfig};
use shared::{
    api::{endpoints::session, ApiEndpoint},
    domain::session::{
        CreateSessionOAuthRequest, CreateSessionOAuthResponse, CreateSessionSuccess,
        GetOAuthUrlResponse, GetOAuthUrlServiceKind, OAuthUrlKind,
    },
};
use sqlx::PgPool;
use url::Url;

use crate::{
    error::{self, ServiceKind},
    extractor::EmailBasicUser,
    google::{self, oauth_url},
    jwk,
    token::{create_oauth_signup_token, create_signin_token, OAuthProvider, TokenSource},
};

#[api_v2_operation]
async fn get_oauth_url(
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
        .ok_or(error::Service::DisabledService(ServiceKind::GoogleOAuth))?;

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

/// Login with basic authorization.
#[api_v2_operation]
async fn create_session(
    settings: Data<RuntimeSettings>,
    user: EmailBasicUser,
) -> Result<HttpResponse, error::Server> {
    // todo: make sure there isn't anything that needs to be done with the db? (eg, actually creating a session id or smth)

    let (csrf, cookie) = create_signin_token(
        user.id,
        &settings.token_secret,
        settings.is_local(),
        TokenSource::Basic,
        settings.login_token_valid_duration,
    )?;

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(CreateSessionSuccess { csrf }))
}

#[api_v2_operation]
/// Login with OAuth
/// May return resources for *signing up* if the user doesn't exist.
async fn create_oauth_session(
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

// todo: what happens if the user has a basic auth?
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
) -> Result<(CreateSessionOAuthResponse, Cookie<'static>), error::OAuth> {
    let redirect_url = google::oauth_url(remote_target, redirect_kind);

    let tokens = google::convert_oauth_code(config, code, &redirect_url).await?;

    let claims = jwks.verify_oauth(&tokens.id_token, 3).await?;

    let google_auth = sqlx::query!(
        "select user_id from user_auth_google where google_id = $1",
        &claims.google_id
    )
    .fetch_optional(db)
    .await?;

    let provider = OAuthProvider::Google {
        google_id: claims.google_id,
    };

    let (csrf, cookie) = match &google_auth {
        Some(auth) => create_signin_token(
            auth.user_id,
            token_secret,
            local_insecure,
            TokenSource::OAuth(provider),
            login_token_valid_duration,
        )?,
        None => create_oauth_signup_token(&claims.email, token_secret, local_insecure, provider)?,
    };

    let response = match google_auth {
        Some(_) => CreateSessionOAuthResponse::Login { csrf },
        None => CreateSessionOAuthResponse::CreateUser { csrf },
    };

    Ok((response, cookie))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        session::GetOAuthUrl::PATH,
        session::GetOAuthUrl::METHOD.route().to(get_oauth_url),
    )
    .route(
        session::Create::PATH,
        session::Create::METHOD.route().to(create_session),
    )
    .route(
        session::CreateOAuth::PATH,
        session::CreateOAuth::METHOD
            .route()
            .to(create_oauth_session),
    );
}
