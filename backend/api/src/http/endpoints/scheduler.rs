use actix_web::{
    web::{method, Data, ServiceConfig},
    HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    algolia::Manager,
    error,
    extractor::UserAgent,
    jwk::{IdentityClaims, JwkVerifier},
    service::{upload::cleaner::UploadCleaner, ServiceData},
};

async fn batch_update(
    algolia_manager: ServiceData<Manager>,
    bearer_auth: BearerAuth,
    jwks: Data<JwkVerifier>,
    user_agent: UserAgent,
) -> Result<HttpResponse, error::ServiceSession> {
    if user_agent
        .0
        .map_or(true, |it| it != "Google-Cloud-Scheduler")
    {
        return Err(error::ServiceSession::Unauthorized);
    }

    let _claims: IdentityClaims = jwks
        .verify_iam_api_invoker_oauth(bearer_auth.token(), 3)
        .await?;

    algolia_manager.spawn_cron_jobs().await?;

    Ok(HttpResponse::Ok().finish())
}

async fn media_clean(
    media_upload_cleaner: ServiceData<UploadCleaner>,
    bearer_auth: BearerAuth,
    jwks: Data<JwkVerifier>,
    user_agent: UserAgent,
) -> Result<HttpResponse, error::ServiceSession> {
    if user_agent
        .0
        .map_or(true, |it| it != "Google-Cloud-Scheduler")
    {
        return Err(error::ServiceSession::Unauthorized);
    }

    let _claims: IdentityClaims = jwks
        .verify_iam_api_invoker_oauth(bearer_auth.token(), 3)
        .await?;

    media_upload_cleaner.spawn_cron_jobs().await?;

    Ok(HttpResponse::Ok().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        "/v1/scheduler/update-algolia",
        method(http::Method::POST).to(batch_update),
    );
    cfg.route(
        "/v1/scheduler/media-clean",
        method(http::Method::POST).to(media_clean),
    );
}
