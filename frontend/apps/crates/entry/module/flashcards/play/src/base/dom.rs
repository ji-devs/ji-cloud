use components::module::_common::play::prelude::DomRenderable;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use components::backgrounds;
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use utils::prelude::*;
use super::{
    state::{Base, Phase},
    game::dom::render as render_game,
    ending::dom::render as render_ending
};


impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(backgrounds::dom::render_raw_single(&state.background, state.theme_id, None))
            .child_signal(state.phase.signal_cloned().map(|phase| {
                match phase {
                    Phase::Init => None,
                    Phase::Playing(game) => Some(render_game(game)),
                    Phase::Ending(ending) => Some(render_ending(ending))
                }
            }))
            //.child(state.instructions.render(&state.audio_mixer))
        })
    }
}
