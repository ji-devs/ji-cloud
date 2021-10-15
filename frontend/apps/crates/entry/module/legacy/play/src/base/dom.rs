use super::state::*;
use components::module::_common::play::prelude::DomRenderable;
use dominator::{html, Dom};
use std::rc::Rc;

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .children(&mut [
                state.render_design()
            ])
        })
    }
}
