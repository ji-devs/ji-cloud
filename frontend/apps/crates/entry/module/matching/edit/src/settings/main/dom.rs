use dominator::{clone, html, Dom};
use shared::domain::module::body::_groups::cards::{get_longest_card_text_length, Card};
use std::rc::Rc;

use super::state::*;
use components::module::_groups::cards::{
    lookup::Side,
    play::card::dom::{
        render_card, render_empty_card, CardOptions, EmptyCardOptions, EmptyKind, Size,
    },
};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};

pub fn render(state: Rc<MainSettings>) -> Dom {
    html!("matching-main", {
        .prop("slot", "main")
        .children_signal_vec(render_top_choices(state.clone()))
        .children_signal_vec(render_bottom_choices(state))

    })
}

fn as_signal_vec(
    sig: impl Signal<Item = Vec<(Card, Side)>>,
) -> impl SignalVec<Item = (Card, Side, usize)> {
    map_ref! {
        let choices = sig
            => {
                let longest_card_text = get_longest_card_text_length(choices.iter().map(|choice| &choice.0));

                choices.into_iter().map(|(card, side)| {
                    (card.clone(), side.clone(), longest_card_text)
                }).collect::<Vec<_>>()
            }
    }.to_signal_vec()
}

fn render_top_choices(state: Rc<MainSettings>) -> impl SignalVec<Item = Dom> {
    as_signal_vec(state.top_choices_signal())
        .map_signal(clone!(state => move |choice| {
            let mode = state.base.mode;
            //theme_id won't have actually changed here, but w/e
            state.base.theme_id.signal_cloned()
                .map(move |theme_id| {
                    html!("matching-column", {
                        .prop("slot", "top")
                        .child({
                            let (card, side, len) = &choice;
                            let mut options = CardOptions::new(card, theme_id, mode, *side, Size::Matching);
                            options.card_text_len = Some(*len);
                            options.flipped = true;
                            render_card(options)
                        })
                        .child({
                            let options = EmptyCardOptions::new(EmptyKind::Question, theme_id, Size::Matching);
                            render_empty_card(options)
                        })
                    })
                })
        }))
}
fn render_bottom_choices(state: Rc<MainSettings>) -> impl SignalVec<Item = Dom> {
    as_signal_vec(state.bottom_choices_signal()).map_signal(clone!(state => move |choice| {
        let mode = state.base.mode;
        //theme_id won't have actually changed here, but w/e
        state.base.theme_id.signal_cloned()
            .map(move |theme_id| {
                let (card, side, len) = &choice;
                let mut options = CardOptions::new(card, theme_id, mode, *side, Size::Matching);
                options.card_text_len = Some(*len);
                options.flipped = true;
                options.slot = Some("bottom");
                render_card(options)
            })
    }))
}
/*
*      <module-page-grid-resize>
       <matching-main slot="main" ${argsToAttrs(props)} ${style} >
         ${mapToString(arrayCount(nPairs), idx => {
           return renderTop(idx);
         })}
         ${mapToString(arrayCount(nPairs), idx => {
           return renderBottom(idx);
         })}
         ${renderFloating()}
       </matching-main>
     </module-page-grid-resize>
     */
