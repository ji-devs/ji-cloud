use crate::algolia::Manager;
use crate::{error::{self, ServiceKind}};

#[get("/v1/update_algolia")]
pub async fn batch_update(
    // algolia_manager: &algolia::Manager,
    algolia_manager: Manager,
    // maybe add a modulus here?
) -> Result<(), error::Service> {

    algolia_manager.spawn_cron_jobs();

    Ok(())
}