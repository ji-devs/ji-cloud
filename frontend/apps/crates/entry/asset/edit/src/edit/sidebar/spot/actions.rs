use super::jig::actions as jig_spot_actions;
use super::playlist::actions as playlist_spot_actions;
use super::state::SpotState;
use super::{
    course::actions as course_actions, jig::actions as jig_actions,
    playlist::actions as playlist_actions,
};
use crate::edit::sidebar::{
    dragging::state::State as DragState,
    state::{SidebarSpot, SidebarSpotItem},
};
use dominator::clone;
use shared::domain::asset::Asset;
use shared::domain::module::ModuleKind;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use utils::init::analytics;
use utils::prelude::*;

#[allow(dead_code)] // this should be removed eventually
pub fn mouse_down(state: Rc<SpotState>, x: i32, y: i32) {
    state
        .sidebar
        .drag
        .set(Some(Rc::new(DragState::new(state.clone(), x, y))));
}

pub fn add_empty_module_after(state: Rc<SpotState>) {
    state
        .sidebar
        .asset_edit_state
        .sidebar_spots
        .lock_mut()
        .insert_cloned(
            state.index + 1,
            SidebarSpot::new_empty(
                &state.sidebar.asset_edit_state.asset_id,
                Some(state.index + 1),
            ),
        );

    match state.sidebar.asset_edit_state.asset_id {
        shared::domain::asset::AssetId::JigId(_) => {
            state
                .sidebar
                .asset_edit_state
                .set_route_jig(JigEditRoute::Landing);
        }
        shared::domain::asset::AssetId::CourseId(_) => {
            state
                .sidebar
                .asset_edit_state
                .target_index
                .set(Some(state.index));

            state
                .sidebar
                .asset_edit_state
                .set_route_course(CourseEditRoute::Unit(None));
        }
        shared::domain::asset::AssetId::PlaylistId(_) => {
            // No op
        },
        shared::domain::asset::AssetId::ResourceId(_) => unreachable!(),
    }
}

pub enum MoveTarget {
    Up,
    Down,
}

pub fn move_index(state: Rc<SpotState>, move_target: MoveTarget) {
    state.sidebar.loader.load(clone!(state => async move {
        if let Some(target) = {
            match move_target {
                MoveTarget::Up if state.index > 1 => {
                    Some(state.index-1)
                },
                MoveTarget::Down if state.index < state.total_len-1 => {
                    Some(state.index+1)
                },
                _ => None
            }
        } {
            state.sidebar.asset_edit_state.sidebar_spots.lock_mut().move_from_to(state.index, target);

            match &state.spot.item {
                SidebarSpotItem::Jig(module) => {
                    jig_actions::update_module_index(
                        Rc::clone(&state),
                        module.as_ref().unwrap_ji(),
                        target as u16
                    ).await;
                },
                SidebarSpotItem::Playlist(_) => {
                    playlist_actions::save_playlist(&state).await;
                },
                SidebarSpotItem::Course(unit) => {
                    course_actions::update_unit_index(
                        Rc::clone(&state),
                        unit.as_ref(),
                        (target - 1) as u16
                    ).await;
                },
            }
        }
    }));
}

pub fn delete(state: Rc<SpotState>) {
    state.sidebar.loader.load(clone!(state => async move {
        state.sidebar.asset_edit_state.sidebar_spots.lock_mut().remove(state.index);

        match &state.spot.item {
            SidebarSpotItem::Jig(module) => {
                jig_actions::delete(&state, &module).await;
            },
            SidebarSpotItem::Playlist(_) => {
                playlist_actions::save_playlist(&state).await;
            },
            SidebarSpotItem::Course(unit) =>
            {
                course_actions::delete(&state, &unit).await;
            },
        }
    }));
}

pub fn assign_to_empty_spot(state: &Rc<SpotState>, data: String) {
    log::info!("data: {}", data);
    state.sidebar.loader.load(clone!(state => async move {
        if state.spot.item.is_none() {
            let spot = if let Ok(kind) = ModuleKind::from_str(&data) {
                let mut properties = HashMap::new();
                properties.insert("Activity Kind", data.to_owned());
                analytics::event("Jig Edit Add Activity", Some(properties));

                jig_spot_actions::assign_module_to_empty_spot( &state, kind)
                    .await
            } else if let Ok(asset) = serde_json::from_str::<Asset>(&data) {
                let mut properties = HashMap::new();
                properties.insert("Activity Kind", format!("Added asset {}", asset.display_name()));
                analytics::event("Playlist Edit Add Activity", Some(properties));

                Some(
                    playlist_spot_actions::assign_asset_to_empty_spot(
                        &state,
                        asset
                    ).await
                )
            }
            // else if let Ok(unit) = ModuleKind::from_str(&data) {
            //     let mut properties = HashMap::new();
            //     properties.insert("Activity Kind", format!("Added asset {}", asset.display_name()));
            //     analytics::event("Playlist Edit Add Activity", Some(properties));

            //     Some(
            //         playlist_spot_actions::assign_unit_to_empty_spot(
            //             &state,
            //             asset
            //         ).await
            //     )
            // }
            else {
                None
            };

            if let Some(module) = spot {

                log::info!("module value: {:?}", module);
                // Instead of replacing the module at the index, we remove the old module and
                // add the new one. This is slightly less efficient because it fires signals
                // for the entire list of modules, however, it is necessary so that the modules
                // before and after this one can have their views updated.
                log::info!("index: {:?}", state.index);

                let mut modules = state.sidebar.asset_edit_state.sidebar_spots.lock_mut();
                modules.remove(state.index);
                modules.insert_cloned(state.index, module);

                // Only add a new placeholder module once the above request has completed and
                // the new module has been added to the list of modules.
                let placeholder_exists = {
                    match modules.last() {
                        // If the list of modules is not empty and the last module is None, then it is
                        // a placeholder module.
                        Some(module) => module.item.is_none(),
                        // When the list is empty or the last module is not a placeholder module.
                        _ => false,
                    }
                };

                // if this is the empty module at the end
                if !placeholder_exists && !state.sidebar.asset_edit_state.asset_id.is_course_id() {
                    modules.push_cloned(SidebarSpot::new_empty(&state.sidebar.asset_edit_state.asset_id, None));
                }

                // jigs are already saved in `assign_module_to_empty_spot`,
                // but playlists saving needs to run after inserting into spots.
                if state.sidebar.asset_edit_state.asset_id.is_playlist_id() {
                    // drop modules because save_playlist uses it
                    drop(modules);
                    playlist_spot_actions::save_playlist(&state).await;
                }

            };
        }
    }));
}
