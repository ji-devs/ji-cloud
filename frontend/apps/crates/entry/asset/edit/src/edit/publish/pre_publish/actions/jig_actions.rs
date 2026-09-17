use shared::{
    api::endpoints::jig,
    domain::jig::{JigGetDraftPath, JigId, JigPublishPath, JigResponse, JigUpdateDraftDataPath},
    error::IntoAnyhow,
};
use utils::init::user::refresh;
use utils::prelude::ApiEndpointExt;

use utils::editable_asset::EditableJig;

pub async fn save_jig(jig: &EditableJig) -> anyhow::Result<()> {
    let req = jig.to_jig_update_request();

    jig::UpdateDraftData::api_with_auth(JigUpdateDraftDataPath(jig.id), Some(req))
        .await
        .into_anyhow()
}

pub async fn publish_jig(jig_id: JigId) -> anyhow::Result<Option<JigResponse>> {
    let (response, status) = jig::Publish::api_with_auth_status(JigPublishPath(jig_id), None).await;
    if status == 402 {
        utils::paywall::dialog_limit(
            "Wanting to create more than 3 JIGs? Upgrade to Pro to publish unlimited JIGs.",
        );
        return Ok(None);
    }
    utils::fetch::side_effect_status_code(status).await;
    response?;
    refresh().await;

    let jig = jig::GetDraft::api_with_auth(JigGetDraftPath(jig_id), None).await?;

    Ok(Some(jig))
}
