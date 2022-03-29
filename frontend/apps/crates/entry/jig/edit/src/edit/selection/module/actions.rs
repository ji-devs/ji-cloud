use std::rc::Rc;

use super::state::State;
use shared::domain::jig::ModuleKind;
use utils::{drag::Drag, unwrap::UnwrapJiExt};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CustomEvent, CustomEventInit, HtmlElement};

impl State {
    pub fn on_pointer_down(self: &Rc<Self>, elem: &HtmlElement, x: i32, y: i32) {
        let drag = Drag::new_anchor_element_resize(x, y, elem, true);
        self.drag.set(Some(Rc::new(drag)));
    }

    pub fn on_pointer_move(self: &Rc<Self>, drag: &Rc<Drag>, x: i32, y: i32) {
        drag.update(x, y);
        trigger_enter_leave_events(&self, x as f32, y as f32);
    }

    pub fn on_pointer_up(self: &Rc<Self>, x: i32, y: i32) {
        self.stop_drag();
        trigger_drop(&self.kind, x as f32, y as f32);
    }

    pub fn stop_drag(self: &Rc<Self>) {
        *self.element_hovered.borrow_mut() = None;
        self.drag.set(None);
    }
}

fn trigger_enter_leave_events(state: &Rc<State>, x: f32, y: f32) {
    let current_elem = element_from_point(x, y);
    let mut previous_elem = state.element_hovered.borrow_mut();

    // if still over same element: do nothing
    if let Some(previous_elem) = &*previous_elem {
        if let Some(current_elem) = &current_elem {
            if previous_elem == current_elem {
                return;
            }
        }
    }

    if let Some(previous_elem) = &*previous_elem {
        let event = create_event("custom-drag-leave");
        let _ = previous_elem.dispatch_event(&event);
    }

    if let Some(current_elem) = &current_elem {
        let event = create_event("custom-drag-enter");
        let _ = current_elem.dispatch_event(&event);
    }

    *previous_elem = current_elem;
}

fn trigger_drop(kind: &ModuleKind, x: f32, y: f32) {
    let mut options = CustomEventInit::new();
    options.detail(&JsValue::from_str(kind.as_str()));
    let event = CustomEvent::new_with_event_init_dict("custom-drop", &options).unwrap_ji();

    if let Some(elem) = element_from_point(x, y) {
        let _ = elem.dispatch_event(&event);
    }
}

fn element_from_point(x: f32, y: f32) -> Option<HtmlElement> {
    web_sys::window()
        .unwrap_ji()
        .document()
        .unwrap_ji()
        .element_from_point(x, y)
        .map(|elem| elem.dyn_into().unwrap_ji())
}

fn create_event(name: &str) -> CustomEvent {
    CustomEvent::new(name).unwrap_ji()
}
