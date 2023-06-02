use super::super::super::spot::state::SpotState;
use crate::edit::sidebar::{state::SidebarSpotItem, CourseSpot};
use shared::{api::endpoints, domain::course::unit::*};
use std::rc::Rc;
use utils::prelude::*;

pub fn edit(state: Rc<SpotState>) {
    let course_id = *state.sidebar.asset_edit_state.asset_id.unwrap_course();

    if let SidebarSpotItem::Course(Some(unit)) = &state.spot.item {
        let unit_id = match &**unit {
            CourseSpot::Cover(_) => None,
            CourseSpot::Unit(unit) => Some(unit.id),
        };

        state
            .sidebar
            .asset_edit_state
            .route
            .set(AssetEditRoute::Course(
                course_id,
                CourseEditRoute::Unit(unit_id),
            ));

        Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
            course_id,
            CourseEditRoute::Unit(unit_id),
        ))));
    };
}

pub async fn delete(state: &Rc<SpotState>, item: &Option<Rc<CourseSpot>>) {
    if let Some(spot) = item {
        let unit_id = match &**spot {
            CourseSpot::Cover(_) => unimplemented!(),
            CourseSpot::Unit(item) => item.id,
        };

        endpoints::course::unit::Delete::api_with_auth_empty(
            DeleteCourseUnitPath(
                state
                    .sidebar
                    .asset_edit_state
                    .asset_id
                    .unwrap_course()
                    .to_owned(),
                unit_id,
            ),
            None,
        )
        .await
        .unwrap_ji();
    }
}

pub async fn update_unit_index(state: Rc<SpotState>, item: Option<&Rc<CourseSpot>>, index: u16) {
    let req = CourseUnitUpdateRequest {
        index: Some(index),
        description: None,
        display_name: None,
        value: None,
    };

    if let Some(item) = item.clone() {
        match &**item {
            CourseSpot::Cover(_) => unimplemented!(),
            CourseSpot::Unit(item) => {
                endpoints::course::unit::Update::api_with_auth_empty(
                    UpdateCourseUnitPath(
                        state
                            .sidebar
                            .asset_edit_state
                            .asset_id
                            .unwrap_course()
                            .to_owned(),
                        item.id,
                    ),
                    Some(req),
                )
                .await
                .unwrap_ji();
            }
        }
    };
}
