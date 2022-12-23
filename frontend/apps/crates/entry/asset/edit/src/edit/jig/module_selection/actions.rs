use std::rc::Rc;

use super::ModuleSelection;
use shared::domain::module::ModuleKind;
use utils::drag::Drag;

impl ModuleSelection {
    pub fn on_pointer_move(self: &Rc<Self>, drag: &Rc<Drag<ModuleKind>>, x: i32, y: i32) {
        drag.update(x, y);
    }

    pub fn on_pointer_up(self: &Rc<Self>, drag: &Rc<Drag<ModuleKind>>, x: i32, y: i32) {
        drag.trigger_drop_event(x, y, drag.data.as_str());
        self.stop_drag();
    }

    pub fn stop_drag(self: &Rc<Self>) {
        self.drag.set(None);
    }
}
