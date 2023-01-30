use shared::{
    api::endpoints::jig,
    domain::jig::{JigGetDraftPath, JigId, JigPublishPath, JigResponse, JigUpdateDraftDataPath},
};
use utils::prelude::ApiEndpointExt;

use utils::editable_asset::EditableJig;

pub async fn save_jig(jig: &EditableJig) -> anyhow::Result<()> {
    let req = jig.to_jig_update_request();

    jig::UpdateDraftData::api_with_auth_empty(JigUpdateDraftDataPath(jig.id), Some(req)).await
}

pub async fn publish_jig(jig_id: JigId) -> anyhow::Result<JigResponse> {
    jig::Publish::api_with_auth_empty(JigPublishPath(jig_id), None).await?;

    let jig = jig::GetDraft::api_with_auth(JigGetDraftPath(jig_id), None).await?;

    Ok(jig)
}
