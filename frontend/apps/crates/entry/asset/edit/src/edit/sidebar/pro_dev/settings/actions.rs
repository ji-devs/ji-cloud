use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::pro_dev::ProDevUpdateDraftDataPath};
use utils::prelude::ApiEndpointExt;

use super::state::ProDevSettings;

impl ProDevSettings {
    pub fn update_pro_dev_settings(self: &Rc<Self>) {
        let state = self;
        let req = state.get_pro_dev_update_req();

        state.loader.load(clone!(state => async move {
            let _ = endpoints::pro_dev::UpdateDraftData::api_with_auth_empty(
                ProDevUpdateDraftDataPath(state.pro_dev.id),
                Some(req),
            )
            .await;
        }));
    }
}
