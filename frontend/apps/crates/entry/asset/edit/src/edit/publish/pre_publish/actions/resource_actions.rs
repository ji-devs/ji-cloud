use shared::{
    api::endpoints::resource,
    domain::resource::{
        ResourceGetDraftPath, ResourceId, ResourcePublishPath, ResourceResponse,
        ResourceUpdateDraftDataPath,
    },
};
use utils::prelude::ApiEndpointExt;

use utils::editable_asset::EditableResource;

pub async fn save_resource(resource: &EditableResource) -> anyhow::Result<ResourceResponse> {
    let req = resource.to_resource_update_request();

    resource::UpdateDraftData::api_with_auth_empty(
        ResourceUpdateDraftDataPath(resource.id),
        Some(req),
    )
    .await?;

    let resource =
        resource::GetDraft::api_with_auth(ResourceGetDraftPath(resource.id), None).await?;

    Ok(resource)
}

pub async fn publish_resource(resource_id: ResourceId) -> anyhow::Result<()> {
    resource::Publish::api_with_auth_empty(ResourcePublishPath(resource_id), None).await
}
