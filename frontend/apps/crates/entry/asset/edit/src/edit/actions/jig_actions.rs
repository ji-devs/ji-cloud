use shared::{
    api::endpoints::jig,
    domain::jig::{JigGetDraftPath, JigId, JigResponse},
};
use utils::prelude::ApiEndpointExt;

pub async fn load_jig(jig_id: JigId) -> anyhow::Result<JigResponse> {
    jig::GetDraft::api_with_auth(JigGetDraftPath(jig_id), None).await
}
