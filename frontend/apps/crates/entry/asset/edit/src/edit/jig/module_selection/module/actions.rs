use std::rc::Rc;

use super::state::State;
use utils::drag::Drag;
use web_sys::HtmlElement;

impl State {
    pub fn on_pointer_down(self: &Rc<Self>, elem: &HtmlElement, x: i32, y: i32) {
        let drag = Drag::new_anchor_element_resize(x, y, elem, true, ());
        self.drag.set(Some(Rc::new(drag)));
    }

    pub fn on_pointer_move(self: &Rc<Self>, drag: &Rc<Drag<()>>, x: i32, y: i32) {
        drag.update(x, y);
    }

    pub fn on_pointer_up(self: &Rc<Self>, drag: &Rc<Drag<()>>, x: i32, y: i32) {
        drag.trigger_drop_event(x, y, self.kind.as_str());
        self.stop_drag();
    }

    pub fn stop_drag(self: &Rc<Self>) {
        self.drag.set(None);
    }
}
