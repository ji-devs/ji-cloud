use core::settings::RuntimeSettings;

use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use paperclip::actix::{api_v2_operation, web::ServiceConfig};
use shared::{
    api::{endpoints::session, ApiEndpoint},
    domain::session::{CreateSessionSuccess, GetOAuthUrlKind, GetOAuthUrlResponse},
};
use url::Url;

use crate::{
    error::{self, ServiceKind},
    extractor::EmailBasicUser,
    jwt::TokenSource,
};

#[api_v2_operation]
async fn get_oauth_url(
    req: HttpRequest,
    config: Data<RuntimeSettings>,
    kind: Path<GetOAuthUrlKind>,
) -> Result<Json<GetOAuthUrlResponse>, error::Service> {
    let oauth_config = match &config.google_oauth {
        Some(it) => it,
        None => return Err(error::Service::DisabledService(ServiceKind::GoogleOAuth)),
    };

    let route = match kind.into_inner() {
        GetOAuthUrlKind::Register => "user/register-oauth",
        GetOAuthUrlKind::Login => "user/login-oauth",
    };

    let route = format!("{}/{}", config.remote_target().frontend_url(), route);

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

    let (csrf, cookie) = crate::extractor::reply_signin_auth(
        user.id,
        &settings.token_secret,
        settings.is_local(),
        TokenSource::Basic,
    )?;

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(CreateSessionSuccess { csrf }))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        session::GetOAuthUrl::PATH,
        session::GetOAuthUrl::METHOD.route().to(get_oauth_url),
    )
    .route(
        session::Create::PATH,
        session::Create::METHOD.route().to(create_session),
        // )
        // .route(
        //     session::CreateOAuth::PATH,
        //     session::CreateOAuth::METHOD
        //         .route()
        //         .to(create_oauth_session),
    );
}
