use shared::{
    api::endpoints::resource,
    domain::resource::{
        ResourceGetDraftPath, ResourceId, ResourcePublishPath, ResourceResponse,
        ResourceUpdateDraftDataPath,
    },
    error::IntoAnyhow,
};
use utils::prelude::ApiEndpointExt;

use utils::editable_asset::EditableResource;

pub async fn save_resource(resource: &EditableResource) -> anyhow::Result<()> {
    let req = resource.to_resource_update_request();

    resource::UpdateDraftData::api_with_auth(ResourceUpdateDraftDataPath(resource.id), Some(req))
        .await
        .into_anyhow()
}

pub async fn publish_resource(resource_id: ResourceId) -> anyhow::Result<ResourceResponse> {
    resource::Publish::api_with_auth(ResourcePublishPath(resource_id), None).await?;

    let resource =
        resource::GetDraft::api_with_auth(ResourceGetDraftPath(resource_id), None).await?;

    Ok(resource)
}
