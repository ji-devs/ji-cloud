use std::rc::Rc;

use dominator::clone;
use shared::domain::asset::{Asset, AssetId};
use utils::prelude::UnwrapJiExt;
use wasm_bindgen_futures::spawn_local;

use super::state::Publish;

mod course_actions;
mod jig_actions;
mod resource_actions;

impl Publish {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            let asset = state.load_asset().await.unwrap_ji();
            state.asset.set(Some(asset));
        }));
    }
    async fn load_asset(self: &Rc<Self>) -> anyhow::Result<Asset> {
        let asset: Asset = match self.asset_edit_state.asset_id {
            AssetId::JigId(jig_id) => jig_actions::load_jig(jig_id).await?.into(),
            AssetId::ResourceId(resource_id) => {
                resource_actions::load_resource(resource_id).await?.into()
            }
            AssetId::CourseId(course_id) => course_actions::load_course(course_id).await?.into(),
        };
        Ok(asset)
    }
}
