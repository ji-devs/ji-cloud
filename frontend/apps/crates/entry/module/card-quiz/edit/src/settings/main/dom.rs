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
    signal::{Signal, SignalExt, ReadOnlyMutable},
    signal_vec::{SignalVec, SignalVecExt},
};

use shared::domain::jig::module::body::{
    ThemeId,
    ModeExt,
    _groups::cards::{Mode, Step, Card}
};
use rand::prelude::*;

use utils::prelude::*;

pub fn render(state: Rc<MainSettings>) -> Dom {
    html!("card-quiz-main", {
        .property("slot", "main")
        .child_signal(render_top_card(state.clone()).map(|dom| Some(dom)))
        .children_signal_vec(render_choices(state.clone()))
    })
}

fn render_top_card(state: Rc<MainSettings>) -> impl Signal<Item = Dom> {

    let sig = map_ref! {
        //theme_id won't have actually changed here, but w/e
        let theme_id = state.base.theme_id.signal_cloned(),
        let (card, side) = state.correct_signal()
            => (*theme_id, card.clone(), *side)
    };

    let mode = state.base.mode.clone();

    sig
        .map(move |(theme_id, card, side)| {
            let mut options = CardOptions::new(&card, theme_id, mode, side, Size::QuizTarget);
            options.flipped = true;
            options.slot = Some("target");

            render_card(options)
        })
}

fn render_choices(state: Rc<MainSettings>) -> impl SignalVec<Item = Dom> {

    state.choices_signal()
        .map_signal(clone!(state => move |choice| {
            let mode = state.base.mode.clone();
            //theme_id won't have actually changed here, but w/e
            state.base.theme_id.signal_cloned()
                .map(move |theme_id| {
                    let (card, side, is_correct) = &choice;
                    let mut options = CardOptions::new(&card, theme_id, mode, *side, Size::QuizOption);
                    options.flipped = true;
                    options.slot = Some("options");

                    render_card(options)
                })
        }))
}
