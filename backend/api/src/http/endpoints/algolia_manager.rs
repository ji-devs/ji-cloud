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
    service::ServiceData,
};

async fn post(
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

    let claims: IdentityClaims = jwks
        .verify_iam_api_invoker_oauth(bearer_auth.token(), 3)
        .await?;

    // check that claims are from the right target?

    log::info!("{:#?}", claims);
    algolia_manager.spawn_cron_jobs().await?;

    Ok(HttpResponse::Ok().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route("/v1/update-algolia", method(http::Method::POST).to(post));
}
