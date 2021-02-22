use shared::domain::jig::{ModuleId, ModuleKind};
use std::rc::Rc;
use super::state::{State, Module};
use utils::drag::Drag;
use crate::edit::sidebar::dragging::state::State as DragState;

pub fn mouse_down(state: Rc<State>, x: i32, y:i32) {
    state.sidebar.drag.set(Some(Rc::new(DragState::new(state.clone(), x, y))));
}

pub fn assign_kind(state: Rc<State>, kind: ModuleKind) {
    state.module.kind.set_neq(Some(kind));
}

pub fn delete(state:Rc<State>) {
    state.sidebar.modules.lock_mut().remove(state.index);
}
pub fn add_module_after(state:Rc<State>) {
    let index = state.index+1;
    let id = ModuleId(uuid::Uuid::from_u128(0));
    state.sidebar.modules.lock_mut().insert_cloned(index, Rc::new(Module::new(id)))
}
pub enum MoveTarget {
    Up,
    Down,
    Any(usize)
}
pub fn move_index(state: Rc<State>, move_target: MoveTarget) {
    if let Some(target) = {
        match move_target {
            MoveTarget::Up if state.index > 1 => {
                Some(state.index-1)
            },
            MoveTarget::Down if state.index < state.total_len-1 => {
                Some(state.index+1)
            },
            MoveTarget::Any(target) => Some(target),
            _ => None
        }
    } {
        state.sidebar.modules.lock_mut().move_from_to(state.index, target)
    }

    
}
