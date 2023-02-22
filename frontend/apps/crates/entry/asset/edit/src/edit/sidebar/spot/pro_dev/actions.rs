use super::super::super::pro_dev::actions as pro_dev_actions;
use super::super::super::spot::state::SpotState;
use crate::edit::sidebar::{
    state::{SidebarSpot, SidebarSpotItem},
    ProDevSpot,
};
use dominator::clone;
use shared::{api::endpoints, domain::pro_dev::unit::*};
use std::rc::Rc;
use utils::prelude::*;

pub fn edit(state: Rc<SpotState>) {
    let pro_dev_id = *state.sidebar.asset_edit_state.asset_id.unwrap_pro_dev();

    if let SidebarSpotItem::ProDev(Some(unit)) = &state.spot.item {
        let unit_id = match &**unit {
            ProDevSpot::Cover(_) => unimplemented!(),
            ProDevSpot::Item(unit) => unit.id,
        };

        state
            .sidebar
            .asset_edit_state
            .route
            .set(AssetEditRoute::ProDev(
                pro_dev_id,
                ProDevEditRoute::Unit(Some(unit_id)),
            ));

        Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::ProDev(
            pro_dev_id,
            ProDevEditRoute::Unit(Some(unit_id)),
        ))));
    };
}

pub async fn delete(state: &Rc<SpotState>, item: &Option<Rc<ProDevSpot>>) {
    if let Some(spot) = item {
        let unit_id = if let Some(item) = item {
            match &**item {
                ProDevSpot::Cover(_) => unimplemented!(),
                ProDevSpot::Item(item) => item.id,
            }
        } else {
            unreachable!()
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

pub async fn assign_unit_to_spot(
    state: &Rc<SpotState>,
    display_name: String,
    description: String,
    value: ProDevUnitValue,
) -> Option<Rc<SidebarSpot>> {
    // Remove unit highlights whenever a new unit is added to the list.
    let pro_dev_id = *state.sidebar.asset_edit_state.asset_id.unwrap_pro_dev();

    let req = ProDevUnitCreateRequest {
        display_name: display_name.clone(),
        description: description.clone(),
        value: value.clone(),
    };

    let resp = endpoints::pro_dev::unit::Create::api_with_auth(
        CreateProDevUnitPath(pro_dev_id),
        Some(req),
    )
    .await
    .unwrap_ji();

    let id = resp.id;
    let index = state.index;

    let req = ProDevUnitUpdateRequest {
        index: Some(index.try_into().unwrap_ji()),
        display_name: None,
        description: None,
        value: None,
    };

    pro_dev_actions::update_unit(&pro_dev_id, &id, req)
        .await
        .unwrap_ji();

    state
        .sidebar
        .asset_edit_state
        .set_route_pro_dev(ProDevEditRoute::Unit(Some(id)));
    Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::ProDev(
        pro_dev_id,
        ProDevEditRoute::Unit(Some(id)),
    ))));

    Some(SidebarSpot::new_pro_dev_unit(ProDevUnit {
        id,
        display_name,
        description,
        value,
    }))
}

pub async fn update_unit_index(state: &Rc<SpotState>, item: Option<&Rc<ProDevSpot>>, index: u16) {
    let req = ProDevUnitUpdateRequest {
        index: Some(index),
        description: None,
        display_name: None,
        value: None,
    };

    let pro_dev_id = *state.sidebar.asset_edit_state.asset_id.unwrap_pro_dev();

    if let Some(item) = item.clone() {
        match &**item {
            ProDevSpot::Cover(_) => unimplemented!(),
            ProDevSpot::Item(item) => {
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
