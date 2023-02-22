use shared::{
    api::endpoints::pro_dev,
    domain::pro_dev::{
        ProDevGetDraftPath, ProDevId, ProDevPublishPath, ProDevResponse, ProDevUpdateDraftDataPath,
    },
};
use utils::prelude::ApiEndpointExt;

use utils::editable_asset::EditableProDev;

pub async fn save_pro_dev(pro_dev: &EditableProDev) -> anyhow::Result<()> {
    let req = pro_dev.to_pro_dev_update_request();

    pro_dev::UpdateDraftData::api_with_auth_empty(ProDevUpdateDraftDataPath(pro_dev.id), Some(req))
        .await
}

pub async fn publish_pro_dev(pro_dev_id: ProDevId) -> anyhow::Result<ProDevResponse> {
    pro_dev::Publish::api_with_auth_empty(ProDevPublishPath(pro_dev_id), None).await?;

    let pro_dev = pro_dev::GetDraft::api_with_auth(ProDevGetDraftPath(pro_dev_id), None).await?;

    Ok(pro_dev)
}
