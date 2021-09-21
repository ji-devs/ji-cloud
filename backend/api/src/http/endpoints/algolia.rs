use crate::algolia::Manager;
use crate::{error::{self, ServiceKind}};

use shared::{
    api::{endpoints::algolia, ApiEndpoint},
};

use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};

async fn get(
    // algolia_manager: &algolia::Manager,
    algolia_manager: Manager,
    // maybe add a modulus here?
) -> Result<(), error::Service> {

    algolia_manager.spawn_cron_jobs().await?;

    Ok(())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        algolia::Get::PATH,
        algolia::Get::METHOD.route().to(get),
    );
}