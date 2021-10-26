use super::state::*;
use components::module::_common::play::prelude::DomRenderable;
use dominator::{html, Dom};
use std::rc::Rc;

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .style("user-select", "none")
            .children(&mut [
                state.clone().render_design(),
                state.clone().render_activity(),
            ])
        })
    }
}
