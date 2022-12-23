use std::rc::Rc;

use super::ModuleSelectionItem;
use utils::drag::Drag;
use web_sys::HtmlElement;

impl ModuleSelectionItem {
    pub fn on_pointer_down(self: &Rc<Self>, elem: &HtmlElement, x: i32, y: i32) {
        let drag = Drag::new_anchor_element_resize(x, y, elem, true, self.kind);
        self.module_selection_state.drag.set(Some(Rc::new(drag)));
    }
}
