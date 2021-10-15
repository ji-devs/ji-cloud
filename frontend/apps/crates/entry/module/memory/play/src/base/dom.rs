use components::{
    backgrounds::dom::render_single_background_raw, module::_common::play::prelude::DomRenderable,
};
use dominator::{html, Dom};
use std::rc::Rc;

use super::{sidebar::dom::render as render_sidebar, stage::dom::render as render_stage, state::*};

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(
                html!("memory-container", {
                    .children(&mut [
                        render_single_background_raw(&state.background, state.theme_id, Some("bg")),
                        render_stage(state.clone()),
                        render_sidebar(state.clone()),
                    ])
                })
            )
        })
    }
}
