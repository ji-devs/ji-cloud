use std::rc::Rc;

use shared::{
    api::endpoints::jig,
    domain::{
        jig::{JigGetDraftPath, JigId, JigResponse},
        module::ModuleKind,
    },
};
use utils::prelude::ApiEndpointExt;

use crate::edit::{sidebar::SidebarSpot, AssetEditState};

pub async fn load_jig(jig_id: JigId) -> anyhow::Result<JigResponse> {
    jig::GetDraft::api_with_auth(JigGetDraftPath(jig_id), None).await
}

impl AssetEditState {
    pub fn get_jig_spots(&self, jig: &JigResponse) {
        let mut add_cover = false;

        if !matches!(
            jig.jig_data.modules.get(0),
            Some(cover) if cover.kind == ModuleKind::Cover
        ) {
            // not cover exists
            add_cover = true;
        };

        let mut modules: Vec<Rc<SidebarSpot>> = jig
            .jig_data
            .modules
            .iter()
            .map(|module| SidebarSpot::new_jig_module(Some(module.clone())))
            .collect();

        if add_cover {
            modules.insert(0, SidebarSpot::new_jig_module(None));
        };

        let mut spots = self.sidebar_spots.lock_mut();
        for module in modules {
            spots.push_cloned(module);
        }
    }
}
