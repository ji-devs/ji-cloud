use dominator::{clone, html, Dom};
use shared::domain::module::body::_groups::cards::get_longest_card_text_length;
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
        .prop("slot", "main")
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
    let sig = map_ref! {
        let choices = state.choices_signal().to_signal_cloned()
            => {
                let longest_card_text = get_longest_card_text_length(choices.iter().map(|choice| &choice.0));

                choices.into_iter().map(|(card, side, is_correct)| {
                    (card.clone(), side.clone(), is_correct.clone(), longest_card_text)
                }).collect::<Vec<_>>()
            }
    }.to_signal_vec();

    sig.map_signal(clone!(state => move |choice| {
        let mode = state.base.mode;
        //theme_id won't have actually changed here, but w/e
        state.base.theme_id.signal_cloned()
            .map(move |theme_id| {
                let (card, side, _is_correct, len) = &choice;
                let mut options = CardOptions::new(card, theme_id, mode, *side, Size::QuizOption);
                options.card_text_len = Some(*len);
                options.flipped = true;
                options.slot = Some("options");

                render_card(options)
            })
    }))
}
