use dominator::clone;
use shared::{
    api::endpoints::{module::*, ApiEndpoint},
    domain::{asset::AssetType, module::*},
    error::EmptyError,
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

            let path = GetDraft::PATH
                .replace("{asset_type}", asset_type.as_str())
                .replace("{module_id}", &state.module_id.0.to_string());

            match api_with_auth::<ModuleResponse, EmptyError, ()>(&path, GetDraft::METHOD, None).await {
                Ok(resp) => {
                    state.module_kind.set(Some(resp.module.body.kind()));
                }
                Err(_) => {}
            }

        }));
    }
}
