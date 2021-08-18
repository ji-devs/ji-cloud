use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::{Signal, SignalExt};
use components::{
    image::search::dom::render as render_image_search,
    text_editor::dom::render_controls as render_text_editor,
};

pub fn render_step_3() -> Dom {
    html!("module-sidebar-body", {
        .property("slot", "body")
        .child(
            html!("div", {
                .text(crate::strings::STR_SIDEBAR_TRACE)
            })
        )
    })
}
