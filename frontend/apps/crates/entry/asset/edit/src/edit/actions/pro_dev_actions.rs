use std::rc::Rc;

use shared::{
    api::endpoints::{self},
    domain::{pro_dev::{ProDevGetDraftPath, ProDevId, ProDevResponse}},
};
use utils::prelude::ApiEndpointExt;

use crate::edit::{sidebar::SidebarSpot, AssetEditState};

pub(crate) async fn load_pro_dev(pro_dev_id: &ProDevId) -> anyhow::Result<ProDevResponse> {
    endpoints::pro_dev::GetDraft::api_with_auth(ProDevGetDraftPath(pro_dev_id.clone()), None).await
}

impl AssetEditState {
    pub async fn get_pro_dev_spots(&self, pro_dev: &ProDevResponse) {
        let units: Vec<Rc<SidebarSpot>> = pro_dev
            .pro_dev_data
            .units
            .iter()
            .map(|unit| SidebarSpot::new_pro_dev_unit(unit.clone()))
            .collect();

        let mut spots = self.sidebar_spots.lock_mut();

        for unit in units {
            spots.push_cloned(unit);
        }

        spots.push_cloned(SidebarSpot::new_empty(&pro_dev.id.into()))
    }
}

