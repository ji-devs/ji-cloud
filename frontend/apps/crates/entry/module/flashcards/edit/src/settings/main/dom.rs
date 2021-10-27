use dominator::{clone, html, Dom};
use std::rc::Rc;

use super::state::*;
use components::module::_groups::cards::{
    lookup::Side,
    play::card::dom::{render_card, CardOptions, Size},
};
use futures_signals::signal::SignalExt;

use shared::domain::jig::module::body::flashcards::DisplayMode;

pub fn render(state: Rc<MainSettings>) -> Dom {
    html!("flashcards-main", {
        .property("slot", "main")
        .children_signal_vec(
            state.display_mode
                .signal()
                .map(clone!(state => move |display_mode| {
                    let mut children:Vec<Dom> = Vec::new();
                    let (card, other, side) = {
                        if state.get_random::<bool>() {
                            (&state.left, &state.right, Side::Left)
                        } else {
                            (&state.right, &state.left, Side::Right)
                        }
                    };


                    let theme_id = state.base.theme_id.get_cloned();
                    let mode = state.base.mode;

                    if display_mode == DisplayMode::Single {

                        let mut options = CardOptions::new(card, theme_id, mode, side, Size::Flashcards);
                        options.back_card = Some(other);
                        options.flip_on_hover = true;
                        options.flipped = true;

                        children.push(render_card(options));
                    } else {
                        let mut options = CardOptions::new(card, theme_id, mode, side, Size::Flashcards);
                        options.flipped = true;

                        children.push(render_card(options));

                        let mut options = CardOptions::new(card, theme_id, mode, side, Size::Flashcards);
                        options.flip_on_hover = true;

                        children.push(render_card(options));
                    }

                    children
                }))
                .to_signal_vec()
        )
    })
}
