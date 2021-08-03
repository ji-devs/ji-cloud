use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use components::{
    image::search::dom::render as render_image_search,
    color_select::dom::render as render_color_picker,
    theme_selector::dom::render_design as render_theme_selector
};

pub fn render(state: Rc<Step1>) -> Dom {
    html!("module-sidebar-body", {
        .property("slot", "body")
        .child(render_theme_selector(state.theme_selector.clone(), None))
    })
}

