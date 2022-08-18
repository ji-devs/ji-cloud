use shared::{
    api::endpoints::{jig, ApiEndpoint},
    domain::jig::{JigId, JigResponse, JigUpdateDraftDataRequest},
    error::{EmptyError, MetadataNotFound},
};
use utils::{
    prelude::{api_with_auth, api_with_auth_empty},
    routes::{AssetEditRoute, AssetRoute, JigEditRoute, Route},
};

use crate::edit::publish::editable_assets::EditableAsset;

use super::super::editable_assets::EditableJig;

pub async fn save_and_publish_jig(jig: &EditableJig) -> Result<(), ()> {
    let path = jig::UpdateDraftData::PATH.replace("{id}", &jig.id.0.to_string());
    let req = jig.to_jig_update_request();
    api_with_auth_empty::<MetadataNotFound, JigUpdateDraftDataRequest>(
        &path,
        jig::UpdateDraftData::METHOD,
        Some(req),
    )
    .await
    .map_err(|_| ())?;

    let path = jig::Publish::PATH.replace("{id}", &jig.id.0.to_string());
    api_with_auth_empty::<EmptyError, ()>(&path, jig::Publish::METHOD, None)
        .await
        .map_err(|_| ())?;

    let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
        jig.id,
        JigEditRoute::PostPublish,
    )))
    .into();
    log::info!("{}", url);

    /* this will cause a full refresh - but preserves history
    * see the .future in EditPage too
    dominator::routing::go_to_url(&url);
    */

    Ok(())
}

pub async fn load_jig(jig_id: JigId) -> Result<EditableAsset, ()> {
    let path = jig::GetDraft::PATH.replace("{id}", &jig_id.0.to_string());

    api_with_auth::<JigResponse, EmptyError, ()>(&path, jig::GetDraft::METHOD, None)
        .await
        .map(|jig| EditableAsset::Jig(EditableJig::new(jig)))
        .map_err(|_| ())
}
