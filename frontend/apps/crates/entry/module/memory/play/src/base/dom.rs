use components::module::play::prelude::DomRenderable;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use components::{backgrounds, stickers, traces};
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
            .child(state.instructions.render(&state.audio_ctx))
            .child(
                html!("play-container", {
                    .property("theme", state.theme_id.as_str_id())
                    .children(&mut [
                        render_stage(state.clone()),
                        render_sidebar(state.clone()),
                    ])
                })
            )
        })
    }
}
