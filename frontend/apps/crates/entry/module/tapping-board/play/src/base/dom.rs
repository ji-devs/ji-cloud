use components::module::play::prelude::DomRenderable;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::state::*;
use components::{backgrounds, stickers, traces};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use super::game::{
    dom::render as render_game,
    state::Game,
};

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .children(&mut [
                backgrounds::dom::render_raw(&state.backgrounds, state.theme_id, None),
                stickers::dom::render_raw(&state.stickers),
                render_game(Game::new(state.clone())),
                state.instructions.render(&state.audio_mixer),
            ])
        })
    }
}
