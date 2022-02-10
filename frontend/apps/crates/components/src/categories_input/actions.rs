use std::rc::Rc;

use dominator::clone;
use js_sys::Reflect;
use utils::unwrap::UnwrapJiExt;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

use super::CategoriesInput;

impl CategoriesInput {
    pub(super) fn on_focus_out(self: &Rc<Self>, input_elem: &HtmlElement) {
        let state = self;
        spawn_local(clone!(state, input_elem => async move {
            // give a chance for the overlay to get focused
            gloo_timers::future::TimeoutFuture::new(0).await;

            let overlay_focused = match &*state.overlay_content_elem.borrow() {
                None => false,
                Some(overlay_content_elem) => focus_within(overlay_content_elem),
            };

            if !focus_within(&input_elem) && !overlay_focused {
                state.focused.set(false);
            };
        }))
    }
}

fn focus_within(elem: &HtmlElement) -> bool {
    let matches_method = Reflect::get(
        &elem,
        &JsValue::from_str("matches")
    ).unwrap_ji();
    let arg = &JsValue::from_str(":focus-within");
    let matches_method = matches_method.dyn_ref::<js_sys::Function>().unwrap_ji();
    let result = matches_method.call1(&elem, arg).unwrap_ji();
    result.as_bool().unwrap_ji()
}
