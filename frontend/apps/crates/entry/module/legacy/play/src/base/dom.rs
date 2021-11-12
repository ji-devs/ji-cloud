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
            .children(&mut [
                state.clone().render_design(),
                state.clone().render_activity(),
            ])
            .event(clone!(state => move |evt:events::Click| {
                state.on_click(evt.mouse_x() as f64, evt.mouse_y() as f64);
            }))
        })
    }
}
