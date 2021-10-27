use components::{
    backgrounds::dom::render_single_background_raw, module::_common::play::prelude::DomRenderable,
};
use dominator::{html, Dom};
use std::rc::Rc;

use super::{
    ending::dom::render as render_ending,
    game::dom::render as render_game,
    state::{Base, Phase},
};
use futures_signals::signal::SignalExt;

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(render_single_background_raw(&state.background, state.theme_id, None))
            .child_signal(state.phase.signal_cloned().map(|phase| {
                match phase {
                    Phase::Init => None,
                    Phase::Playing(game) => Some(render_game(game)),
                    Phase::Ending(ending) => Some(render_ending(ending))
                }
            }))
        })
    }
}
