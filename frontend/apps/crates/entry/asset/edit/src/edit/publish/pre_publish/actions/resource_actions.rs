use shared::{
    api::endpoints::resource,
    domain::resource::{ResourcePublishPath, ResourceUpdateDraftDataPath},
};
use utils::prelude::ApiEndpointExt;

use utils::editable_asset::EditableResource;

pub async fn save_and_publish_resource(resource: &EditableResource) -> Result<(), ()> {
    // let path = resource::UpdateDraftData::PATH.replace("{id}", &resource.id.0.to_string());
    let req = resource.to_resource_update_request();
    resource::UpdateDraftData::api_with_auth_empty(
        ResourceUpdateDraftDataPath(resource.id),
        Some(req),
    )
    .await
    .map_err(|_| ())?;

    // let path = PATH.replace("{id}", &resource.id.0.to_string());
    resource::Publish::api_with_auth_empty(ResourcePublishPath(resource.id), None)
        .await
        .map_err(|_| ())?;

    Ok(())
}
