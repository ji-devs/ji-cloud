use actix_web::{
    http,
    web::{method, Data, ServiceConfig},
    HttpRequest, HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use core::settings::RuntimeSettings;

use crate::{
    error,
    jwk::{IdentityClaims, JwkVerifier},
};

async fn scheduler_iam_test(
    bearer_auth: BearerAuth,
    settings: Data<RuntimeSettings>,
    jwks: Data<JwkVerifier>,
    req: HttpRequest,
) -> Result<HttpResponse, error::Server> {
    let bearer_auth_token = bearer_auth.token();

    log::info!("{}", bearer_auth_token);

    let claims: IdentityClaims = jwks
        .verify_iam_service_account_oauth(bearer_auth_token, 3)
        .await?;

    log::info!("{:#?}", claims);

    Ok(HttpResponse::Ok().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        "/v0/scheduler-test",
        method(http::Method::POST).to(scheduler_iam_test),
    );
}
