use super::state::*;
use super::styles;
use components::module::_common::play::prelude::DomRenderable;
use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::prelude::*;

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("div", {
            .class(&*styles::FULL_STAGE)
            .property("slot", "main")
            .style("user-select", "none")
            .child(html!("img", {
                .class(&*styles::FULL_STAGE)
                .attribute("src", &state.design_media_url(&state.slide.image_full))
            }))
            .apply_if(state.should_render_design(), |dom| {
                dom.child(state.clone().render_design())
            })
            .child(state.clone().render_activity())
            .event(clone!(state => move |evt:events::Click| {
                state.on_click(evt.mouse_x() as f64, evt.mouse_y() as f64);
            }))
        })
    }
}
