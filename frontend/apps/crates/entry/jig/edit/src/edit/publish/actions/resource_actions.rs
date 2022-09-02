use shared::{
    api::endpoints::resource,
    domain::resource::{
        ResourceGetDraftPath, ResourceId, ResourcePublishPath, ResourceUpdateDraftDataPath,
    },
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{AssetEditRoute, AssetRoute, ResourceEditRoute, Route},
};

use crate::edit::publish::editable_assets::EditableAsset;

use super::super::editable_assets::EditableResource;

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

    let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
        resource.id,
        ResourceEditRoute::PostPublish,
    )))
    .into();
    log::info!("{}", url);

    /* this will cause a full refresh - but preserves history
    * see the .future in EditPage too
    dominator::routing::go_to_url(&url);
    */

    Ok(())
}

pub async fn load_resource(resource_id: ResourceId) -> anyhow::Result<EditableAsset> {
    // let path = resource::GetDraft::PATH.replace("{id}", &resource_id.0.to_string());

    resource::GetDraft::api_with_auth(ResourceGetDraftPath(resource_id), None)
        .await
        .map(|resource| EditableAsset::Resource(EditableResource::new(resource)))
}
