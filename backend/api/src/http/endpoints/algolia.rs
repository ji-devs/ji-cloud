use crate::algolia::Manager;
use crate::{error::{self, ServiceKind}};

use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use crate::service::ServiceData;

async fn post(
    algolia_manager: ServiceData<Manager>,
    // TODO: add the modulus turn here as a request
) -> Result<HttpResponse, error::NotFound> {

    algolia_manager.spawn_cron_jobs().await?;

    // TODO: add error here
    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        "/v1/update-algolia",
        actix_web::web::post().to(post),
    );
}