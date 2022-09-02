use shared::{
    api::endpoints::jig,
    domain::jig::{JigGetDraftPath, JigId, JigPublishPath, JigUpdateDraftDataPath},
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{AssetEditRoute, AssetRoute, JigEditRoute, Route},
};

use crate::edit::publish::editable_assets::EditableAsset;

use super::super::editable_assets::EditableJig;

pub async fn save_and_publish_jig(jig: &EditableJig) -> Result<(), ()> {
    let req = jig.to_jig_update_request();

    jig::UpdateDraftData::api_with_auth_empty(JigUpdateDraftDataPath(jig.id.clone()), Some(req))
        .await
        .map_err(|_| ())?;

    jig::Publish::api_with_auth_empty(JigPublishPath(jig.id.clone()), None)
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

pub async fn load_jig(jig_id: JigId) -> anyhow::Result<EditableAsset> {
    jig::GetDraft::api_with_auth(JigGetDraftPath(jig_id), None)
        .await
        .map(|jig| EditableAsset::Jig(EditableJig::new(jig)))
}
