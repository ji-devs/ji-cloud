use shared::{
    api::endpoints::resource,
    domain::resource::{ResourceGetDraftPath, ResourceId, ResourceResponse},
};
use utils::prelude::ApiEndpointExt;

pub async fn load_resource(resource_id: ResourceId) -> anyhow::Result<ResourceResponse> {
    resource::GetDraft::api_with_auth(ResourceGetDraftPath(resource_id), None).await
}
