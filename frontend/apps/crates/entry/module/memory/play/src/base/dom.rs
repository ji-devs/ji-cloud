use components::{
    module::_common::play::prelude::DomRenderable,
    backgrounds::dom::render_single_background_raw
};
use dominator::{html, Dom, clone};
use std::rc::Rc;
use components::backgrounds;
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use utils::prelude::*;
use super::{
    state::*,
    stage::dom::render as render_stage,
    sidebar::dom::render as render_sidebar,
};

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
