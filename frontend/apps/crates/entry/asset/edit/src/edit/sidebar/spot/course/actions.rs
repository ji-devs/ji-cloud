use super::super::super::spot::state::SpotState;
use crate::edit::sidebar::{CourseSpot, ProDevSpot, SidebarSpot, SidebarSpotItem};
use itertools::Itertools;
use shared::{
    api::endpoints,
    domain::{
        asset::Asset,
        course::{CourseUpdateDraftDataPath, CourseUpdateDraftDataRequest},
    },
};
use std::rc::Rc;
use utils::prelude::ApiEndpointExt;

pub async fn save_course(state: &Rc<SpotState>) {
    let items = state
        .sidebar
        .asset_edit_state
        .sidebar_spots
        .lock_ref()
        .iter()
        .filter_map(|spot| {
            // filter out cover and empty spots
            match &spot.item {
                SidebarSpotItem::Jig(_) => unreachable!(),
                SidebarSpotItem::Course(spot) => match spot {
                    None => None,
                    Some(spot) => match &**spot {
                        CourseSpot::Cover(_) => None,
                        CourseSpot::Item(jig) => Some(jig.id),
                    },
                },
                SidebarSpotItem::ProDev(_) => unreachable!(),
            }
        })
        .collect_vec();

    let req = CourseUpdateDraftDataRequest {
        items: Some(items),
        ..Default::default()
    };

    let _ = endpoints::course::UpdateDraftData::api_with_auth_empty(
        CourseUpdateDraftDataPath(*state.sidebar.asset_edit_state.asset_id.unwrap_course()),
        Some(req),
    )
    .await;
}

// pub fn assign_asset_to_empty_spot(_state: &Rc<SpotState>, asset: Asset) -> Rc<SidebarSpot> {
//     let jig = match asset {
//         Asset::Jig(jig) => jig,
//         Asset::Resource(_) => todo!(),
//         Asset::Course(_) => unreachable!(),
//     };
//     SidebarSpot::new_course_item(jig)
// }

pub async fn assign_asset_to_empty_spot(_state: &Rc<SpotState>, asset: Asset) -> Rc<SidebarSpot> {
    let jig = match asset {
        Asset::Jig(jig) => jig,
        Asset::Resource(_) => todo!(),
        Asset::Course(_) => unreachable!(),
        Asset::ProDev(_) => todo!(),
    };
    SidebarSpot::new_course_item(jig)
}
