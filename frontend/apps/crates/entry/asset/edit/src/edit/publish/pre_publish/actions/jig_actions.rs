use shared::{
    api::endpoints::jig,
    domain::jig::{JigPublishPath, JigUpdateDraftDataPath},
};
use utils::prelude::ApiEndpointExt;

use super::super::editable_assets::EditableJig;

pub async fn save_and_publish_jig(jig: &EditableJig) -> Result<(), ()> {
    let req = jig.to_jig_update_request();

    jig::UpdateDraftData::api_with_auth_empty(JigUpdateDraftDataPath(jig.id.clone()), Some(req))
        .await
        .map_err(|_| ())?;

    jig::Publish::api_with_auth_empty(JigPublishPath(jig.id.clone()), None)
        .await
        .map_err(|_| ())?;

    Ok(())
}
