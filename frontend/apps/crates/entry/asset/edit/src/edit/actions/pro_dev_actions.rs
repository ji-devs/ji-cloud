use std::rc::Rc;

use shared::{
    api::endpoints::{self},
    domain::pro_dev::{
        unit::{GetProDevUnitDraftPath, ProDevUnit, ProDevUnitId},
        ProDevGetDraftPath, ProDevId, ProDevResponse,
    },
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

use crate::edit::{sidebar::SidebarSpot, AssetEditState};

pub(crate) async fn load_pro_dev(pro_dev_id: &ProDevId) -> anyhow::Result<ProDevResponse> {
    endpoints::pro_dev::GetDraft::api_with_auth(ProDevGetDraftPath(pro_dev_id.clone()), None).await
}

impl AssetEditState {
    pub async fn get_pro_dev_spots(&self, pro_dev: &ProDevResponse) {
        let mut items = vec![SidebarSpot::new_pro_dev_cover(
            pro_dev.pro_dev_data.cover.clone().unwrap(),
        )];

        for unit in &pro_dev.pro_dev_data.units {
            let unit = get_unit(&pro_dev.id, &unit.id).await;

            items.push(SidebarSpot::new_pro_dev_unit(unit));
        }

        let mut spots = self.sidebar_spots.lock_mut();

        for item in items {
            spots.push_cloned(item);
        }
    }
}

async fn get_unit(pro_dev_id: &ProDevId, unit_id: &ProDevUnitId) -> ProDevUnit {
    endpoints::pro_dev::unit::GetDraft::api_with_auth(
        GetProDevUnitDraftPath(pro_dev_id.clone(), unit_id.clone()),
        None,
    )
    .await
    .unwrap_ji()
}
