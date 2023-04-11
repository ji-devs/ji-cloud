use super::super::super::spot::state::SpotState;
use crate::edit::sidebar::{state::SidebarSpotItem, ProDevSpot, SidebarSpot};
use shared::{api::endpoints, domain::pro_dev::unit::*};
use std::rc::Rc;
use utils::prelude::*;

pub fn edit(state: Rc<SpotState>) {
    let pro_dev_id = *state.sidebar.asset_edit_state.asset_id.unwrap_pro_dev();

    if let SidebarSpotItem::ProDev(Some(unit)) = &state.spot.item {
        let unit_id = match &**unit {
            ProDevSpot::Cover(_) => None,
            ProDevSpot::Unit(unit) => Some(unit.id),
        };

        log::info!("{unit_id:?}");

        state
            .sidebar
            .asset_edit_state
            .route
            .set(AssetEditRoute::ProDev(
                pro_dev_id,
                ProDevEditRoute::Unit(unit_id),
            ));

        Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::ProDev(
            pro_dev_id,
            ProDevEditRoute::Unit(unit_id),
        ))));
    };
}

pub async fn delete(state: &Rc<SpotState>, item: &Option<Rc<ProDevSpot>>) {
    if let Some(spot) = item {
        let unit_id = match &**spot {
            ProDevSpot::Cover(_) => unimplemented!(),
            ProDevSpot::Unit(item) => item.id,
        };

        endpoints::pro_dev::unit::Delete::api_with_auth_empty(
            DeleteProDevUnitPath(
                state
                    .sidebar
                    .asset_edit_state
                    .asset_id
                    .unwrap_pro_dev()
                    .to_owned(),
                unit_id,
            ),
            None,
        )
        .await
        .unwrap();
    }
}

pub async fn update_unit_index(state: &Rc<SpotState>, item: Option<&Rc<ProDevSpot>>, index: u16) {
    let req = ProDevUnitUpdateRequest {
        index: Some(index),
        description: None,
        display_name: None,
        value: None,
    };

    log::info!("spot index {}", index);

    if let Some(item) = item.clone() {
        match &**item {
            ProDevSpot::Cover(_) => unimplemented!(),
            ProDevSpot::Unit(item) => {
                endpoints::pro_dev::unit::Update::api_with_auth_empty(
                    UpdateProDevUnitPath(
                        state
                            .sidebar
                            .asset_edit_state
                            .asset_id
                            .unwrap_pro_dev()
                            .to_owned(),
                        item.id,
                    ),
                    Some(req),
                )
                .await
                .unwrap();
            }
        }
    };
}
