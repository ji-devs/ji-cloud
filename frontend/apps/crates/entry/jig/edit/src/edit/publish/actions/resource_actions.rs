use shared::{
    api::endpoints::{resource, ApiEndpoint},
    domain::resource::{ResourceId, ResourceResponse, ResourceUpdateDraftDataRequest},
    error::{EmptyError, MetadataNotFound},
};
use utils::{
    prelude::{api_with_auth, api_with_auth_empty},
    routes::{AssetEditRoute, AssetRoute, ResourceEditRoute, Route},
};

use crate::edit::publish::editable_assets::EditableAsset;

use super::super::editable_assets::EditableResource;

pub async fn save_and_publish_resource(resource: &EditableResource) -> Result<(), ()> {
    let path = resource::UpdateDraftData::PATH.replace("{id}", &resource.id.0.to_string());
    let req = resource.to_resource_update_request();
    api_with_auth_empty::<MetadataNotFound, ResourceUpdateDraftDataRequest>(
        &path,
        resource::UpdateDraftData::METHOD,
        Some(req),
    )
    .await
    .map_err(|_| ())?;

    let path = resource::Publish::PATH.replace("{id}", &resource.id.0.to_string());
    api_with_auth_empty::<EmptyError, ()>(&path, resource::Publish::METHOD, None)
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

pub async fn load_resource(resource_id: ResourceId) -> Result<EditableAsset, ()> {
    let path = resource::GetDraft::PATH.replace("{id}", &resource_id.0.to_string());

    api_with_auth::<ResourceResponse, EmptyError, ()>(&path, resource::GetDraft::METHOD, None)
        .await
        .map(|resource| EditableAsset::Resource(EditableResource::new(resource)))
        .map_err(|_| ())
}
