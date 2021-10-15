use super::state::*;
use components::{
    backgrounds::dom::render_backgrounds_raw, module::_common::play::prelude::DomRenderable,
};
use dominator::{html, Dom};
use std::rc::Rc;

use super::game::{dom::render as render_game, state::Game};

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .children(&mut [
                render_backgrounds_raw(&state.backgrounds, state.theme_id, None),
                render_game(Game::new(state.clone())),
            ])
        })
    }
}
