use super::state::State;
use crate::edit::sidebar::{dragging::state::State as DragState, state::{SidebarSpotItem, SidebarSpot}};
use std::rc::Rc;
use dominator::clone;
use shared::domain::jig::ModuleKind;
use utils::prelude::*;
use super::jig::actions as jig_spot_actions;

#[allow(dead_code)] // this should be removed eventually
pub fn mouse_down(state: Rc<State>, x: i32, y: i32) {
    state
        .sidebar
        .drag
        .set(Some(Rc::new(DragState::new(state.clone(), x, y))));
}

pub fn add_empty_module_after(state: Rc<State>) {
    state
        .sidebar
        .modules
        .lock_mut()
        .insert_cloned(state.index + 1, Rc::new(SidebarSpot::new_empty(&state.sidebar.jig)));
    state
        .sidebar
        .jig_edit_state
        .route
        .set_neq(JigEditRoute::Landing);
}

pub enum MoveTarget {
    Up,
    Down,
}

pub fn move_index(state: Rc<State>, move_target: MoveTarget) {
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
            state.sidebar.modules.lock_mut().move_from_to(state.index, target);

            match &state.module.item {
                SidebarSpotItem::Jig(module) => {
                    jig_spot_actions::update_module_index(
                        Rc::clone(&state),
                        &module.as_ref().unwrap(),
                        target as u16
                    ).await;
                },
            }
        }
    }));
}

pub fn delete(state: Rc<State>) {
    state.sidebar.loader.load(clone!(state => async move {
        jig_spot_actions::delete(Rc::clone(&state)).await;
        state.sidebar.modules.lock_mut().remove(state.index);
    }));
}

pub fn on_module_kind_drop(state: Rc<State>, module_kind: ModuleKind) {
    if state.index == 0 && module_kind != ModuleKind::Cover {
        return;
    }
    if state.module.item.is_none() {
        jig_spot_actions::assign_kind(state.clone(), module_kind);
    }

    // Remove module highlights whenever a new module is added to the list.
    state.sidebar.highlight_modules.set_neq(None);
}