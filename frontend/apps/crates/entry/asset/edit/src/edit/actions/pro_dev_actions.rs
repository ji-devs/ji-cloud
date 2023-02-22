use std::rc::Rc;

use shared::{
    api::endpoints::{self},
    domain::pro_dev::{ProDevGetDraftPath, ProDevId, ProDevResponse},
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

use crate::edit::{sidebar::SidebarSpot, AssetEditState};

async fn load_pro_dev(pro_dev_id: &ProDevId) -> ProDevResponse {
    endpoints::pro_dev::GetDraft::api_with_auth(ProDevGetDraftPath(pro_dev_id.clone()), None)
        .await
        .unwrap_ji()
}
impl AssetEditState {
    pub async fn get_pro_dev_spots(&self, pro_dev: &ProDevResponse) {
        let mut items = vec![SidebarSpot::new_pro_dev_cover(
            pro_dev.pro_dev_data.cover.clone().unwrap(),
        )];

        let mut unit: Vec<Rc<SidebarSpot>> = pro_dev
            .pro_dev_data
            .units
            .iter()
            .map(|unit| SidebarSpot::new_pro_dev_unit(unit.clone()))
            .collect();

        // add empty at the end
        items.push(SidebarSpot::new_empty(&pro_dev.id.into()));

        let mut spots = self.sidebar_spots.lock_mut();
        for item in items {
            spots.push_cloned(item);
        }
    }
}
