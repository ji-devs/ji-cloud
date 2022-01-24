use super::state::*;
use dominator::{html, Dom};
use std::rc::Rc;

use components::theme_selector::dom::render_design as render_theme_selector;

pub fn render(state: Rc<Step1>) -> Dom {
    html!("module-sidebar-body", {
        .property("slot", "body")
        .child(render_theme_selector(state.theme_selector.clone(), None, None))
    })
}
