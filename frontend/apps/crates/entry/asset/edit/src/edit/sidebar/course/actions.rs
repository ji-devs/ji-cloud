use super::super::state::Sidebar;
use crate::edit::sidebar::SidebarSpot;
use dominator::clone;
use shared::domain::course::unit::CourseUnit;
use shared::{
    api::endpoints,
    domain::course::{CourseId, CourseUpdateDraftDataPath, CourseUpdateDraftDataRequest},
};
use std::rc::Rc;
use utils::prelude::*;

#[allow(dead_code)] // TODO: remove once used
pub fn navigate_to_publish(state: Rc<Sidebar>) {
    state.collapsed.set(true);
    state.asset_edit_state.navigate_to_publish();
}

pub async fn update_course(
    course_id: &CourseId,
    req: CourseUpdateDraftDataRequest,
) -> anyhow::Result<()> {
    endpoints::course::UpdateDraftData::api_with_auth(
        CourseUpdateDraftDataPath(course_id.clone()),
        Some(req),
    )
    .await
    .into_anyhow()
}

pub async fn update_display_name(course_id: CourseId, value: String) {
    let req = CourseUpdateDraftDataRequest {
        display_name: Some(value),
        ..Default::default()
    };

    let _ = update_course(&course_id, req).await;
}

pub fn duplicate_unit(state: Rc<Sidebar>, unit: &CourseUnit) {
    state.loader.load(clone!(state, unit => async move {
        let course_id = state.asset_edit_state.asset_id.unwrap_course();
        let unit = super::unit_cloner::clone_unit(&unit, &course_id).await.unwrap_ji();
        populate_added_unit(state, unit);
    }));
}

fn populate_added_unit(state: Rc<Sidebar>, unit: CourseUnit) {
    let insert_at_idx = state.asset_edit_state.sidebar_spots.lock_ref().len();

    let unit_id = unit.id;

    state
        .asset_edit_state
        .sidebar_spots
        .lock_mut()
        .insert_cloned(insert_at_idx, SidebarSpot::new_course_unit(unit));

    state
        .asset_edit_state
        .set_route_course(CourseEditRoute::Unit(Some(unit_id)));
}
