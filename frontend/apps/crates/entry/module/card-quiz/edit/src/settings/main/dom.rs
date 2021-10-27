use dominator::{clone, html, Dom};
use std::rc::Rc;

use super::state::*;
use components::module::_groups::cards::play::card::dom::{render_card, CardOptions, Size};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};

pub fn render(state: Rc<MainSettings>) -> Dom {
    html!("card-quiz-main", {
        .property("slot", "main")
        .child_signal(render_top_card(state.clone()).map(Some))
        .children_signal_vec(render_choices(state))
    })
}

fn render_top_card(state: Rc<MainSettings>) -> impl Signal<Item = Dom> {
    let sig = map_ref! {
        //theme_id won't have actually changed here, but w/e
        let theme_id = state.base.theme_id.signal_cloned(),
        let (card, side) = state.correct_signal()
            => (*theme_id, card.clone(), *side)
    };

    let mode = state.base.mode;

    sig.map(move |(theme_id, card, side)| {
        let mut options = CardOptions::new(&card, theme_id, mode, side, Size::QuizTarget);
        options.flipped = true;
        options.slot = Some("target");

        render_card(options)
    })
}

fn render_choices(state: Rc<MainSettings>) -> impl SignalVec<Item = Dom> {
    state.choices_signal()
        .map_signal(clone!(state => move |choice| {
            let mode = state.base.mode;
            //theme_id won't have actually changed here, but w/e
            state.base.theme_id.signal_cloned()
                .map(move |theme_id| {
                    let (card, side, _is_correct) = &choice;
                    let mut options = CardOptions::new(card, theme_id, mode, *side, Size::QuizOption);
                    options.flipped = true;
                    options.slot = Some("options");

                    render_card(options)
                })
        }))
}
