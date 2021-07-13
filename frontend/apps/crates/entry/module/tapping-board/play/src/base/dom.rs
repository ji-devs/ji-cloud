use components::{
    module::_common::play::prelude::DomRenderable,
    backgrounds::dom::render_backgrounds_raw,
    stickers::dom::render_stickers_raw
};
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
                render_backgrounds_raw(&state.backgrounds, state.theme_id, None),
                render_stickers_raw(&state.stickers, state.theme_id),
                render_game(Game::new(state.clone())),
            ])
        })
    }
}
