use dominator::clone;
use shared::{
    api::endpoints::module::*,
    domain::{asset::AssetType, module::*},
};
use std::rc::Rc;
use utils::prelude::*;

use super::ModuleIframe;

impl ModuleIframe {
    pub fn load_module_kind(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            //TODO - API to just get module kind, so no need to load entire body here

            let asset_type = AssetType::from(&state.asset_id);

            match GetDraft::api_with_auth(ModuleGetDraftPath(asset_type, state.module_id.clone()), None).await {
                Ok(resp) => {
                    state.module_kind.set(Some(resp.module.body.kind()));
                }
                Err(_) => {}
            }

        }));
    }
}
