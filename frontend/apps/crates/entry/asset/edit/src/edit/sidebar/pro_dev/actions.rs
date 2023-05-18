use super::super::state::Sidebar;
use crate::edit::sidebar::SidebarSpot;
use crate::edit::sidebar::SidebarSpotItem::ProDev;
use dominator::clone;
use shared::domain::pro_dev::unit::ProDevUnit;
use shared::{
    api::endpoints,
    domain::pro_dev::{
        unit::{ProDevUnitId, ProDevUnitUpdateRequest, UpdateProDevUnitPath},
        ProDevId, ProDevUpdateDraftDataPath, ProDevUpdateDraftDataRequest,
    },
};
use std::rc::Rc;
use utils::prelude::*;

#[allow(dead_code)] // TODO: remove once used
pub fn navigate_to_publish(state: Rc<Sidebar>) {
    state.collapsed.set(true);
    state.asset_edit_state.navigate_to_publish();
}

pub async fn update_pro_dev(
    pro_dev_id: &ProDevId,
    req: ProDevUpdateDraftDataRequest,
) -> anyhow::Result<()> {
    endpoints::pro_dev::UpdateDraftData::api_with_auth_empty(
        ProDevUpdateDraftDataPath(pro_dev_id.clone()),
        Some(req),
    )
    .await
}

pub async fn _update_unit(
    pro_dev_id: &ProDevId,
    unit_id: &ProDevUnitId,
    req: ProDevUnitUpdateRequest,
) -> anyhow::Result<()> {
    endpoints::pro_dev::unit::Update::api_with_auth_empty(
        UpdateProDevUnitPath(pro_dev_id.clone(), unit_id.clone()),
        Some(req),
    )
    .await
}

pub async fn update_display_name(pro_dev_id: ProDevId, value: String) {
    let req = ProDevUpdateDraftDataRequest {
        display_name: Some(value),
        ..Default::default()
    };

    let _ = update_pro_dev(&pro_dev_id, req).await;
}

pub fn duplicate_unit(state: Rc<Sidebar>, unit: &ProDevUnit) {
    state.loader.load(clone!(state, unit => async move {
        let pro_dev_id = state.asset_edit_state.asset_id.unwrap_pro_dev();
        let unit = super::unit_cloner::clone_unit(&unit, &pro_dev_id).await.unwrap_ji();
        populate_added_unit(state, unit);
    }));
}

fn populate_added_unit(state: Rc<Sidebar>, unit: ProDevUnit) {
    // Assumes that the final module in the list is always the placeholder module.
    let insert_at_idx = state.asset_edit_state.sidebar_spots.lock_ref().len();

    let unit_id = unit.id;

    state
        .asset_edit_state
        .sidebar_spots
        .lock_mut()
        .insert_cloned(insert_at_idx, SidebarSpot::new_pro_dev_unit(unit));

    state
        .asset_edit_state
        .set_route_pro_dev(ProDevEditRoute::Unit(Some(unit_id)));
}
