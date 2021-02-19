use crate::edit::sidebar::state::State as SidebarState;
use std::rc::Rc;
use super::state::*;

//Mouse movements are triggered from sidebar regardless of
//whether drag state exists yet or not
pub fn mouse_move(sidebar: Rc<SidebarState>, x: i32, y:i32) {
    //update via ref not lock_mut
    //otherwise it will replace the drag and cause a re-render
    //with every update
    //internally, drag uses Mutable and Atomic so this works in Rc
    if let Some(drag) = &*sidebar.drag.lock_ref() {
        drag.inner.update(x, y);
    }
}

pub fn mouse_up(sidebar: Rc<SidebarState>, x: i32, y:i32) {
    if let Some(drag) = sidebar.drag.replace(None) {
        log::info!("stopped drag listener!");
    }
}

pub fn update_index(state: Rc<State>, x: i32, y: i32) {
    let sidebar = &state.module.sidebar;
    //TODO - actually update based on pos
    state.target_index.set_neq(Some(state.module.index));
}
