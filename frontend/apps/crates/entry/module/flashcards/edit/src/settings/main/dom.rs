use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::{
    state::*,
    settings::state::*
};
use super::state::*;
use components::module::_groups::cards::{
    lookup::{self, Side},
    play::card::dom::{render_card, CardOptions, Size},
    edit::{
        config,
        state::*
    },
};
use futures_signals::{
    map_ref,
    signal::{SignalExt, ReadOnlyMutable}
};

use shared::domain::jig::module::body::{
    ThemeId,
    ModeExt,
    _groups::cards::{Mode, Step, Card}
};
use rand::prelude::*;

use utils::prelude::*;

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
                    let mode = state.base.mode.clone();

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
