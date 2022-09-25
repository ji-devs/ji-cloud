use crate::edit::sidebar::state::State as SidebarState;
use std::rc::Rc;

//Mouse movements are triggered from sidebar regardless of
//whether drag state exists yet or not
pub fn mouse_move(sidebar: Rc<SidebarState>, x: i32, y: i32) {
    //update via ref not lock_mut
    //otherwise it will replace the drag and cause a re-render
    //with every update
    //internally, drag uses Mutable and Atomic so this works in Rc
    if let Some(drag) = &*sidebar.drag.lock_ref() {
        drag.inner.update(x, y);
    }
}

pub fn mouse_up(sidebar: Rc<SidebarState>, _x: i32, _y: i32) {
    if let Some(_drag) = sidebar.drag.replace(None) {
        sidebar.drag_target_index.set_neq(None);
    }
}
