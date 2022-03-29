use crate::base::{card::state::*, state::*};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

use components::module::_groups::cards::play::card::dom::{render_card_mixin, CardOptions, Size};

pub fn render(state: Rc<Base>) -> Dom {
    html!("play-sidebar", {
        .property("slot", "sidebar")
        .children(

            //It's simpler to just always render but hide via CSS until we start the animation
            state.cards
                .iter()
                .map(clone!(state => move |card| {
                    render_sidebar_card(state.clone(), card.clone())
                }))
        )
    })
}

fn render_sidebar_card(state: Rc<Base>, card_state: Rc<CardState>) -> Dom {
    let _card_id = &card_state.id;
    let card = &card_state.card;

    let theme_id = state.theme_id;
    let mode = state.mode;
    let side = card_state.side;
    let size = Size::Memory;

    let options = CardOptions::new(card, theme_id, mode, side, size);

    render_card_mixin(options, |dom| {
        dom.style_signal(
            "display",
            card_state
                .is_found()
                .map(|found| if !found { "none" } else { "block" }),
        )
        .property("flipped", true)
        .apply(clone!(card_state => move |dom| {
            dom
                .style_signal("transform", {
                    card_state.animation_state_signal()
                        .map(|animation_state| {
                            match animation_state {
                                Some(animation_state) => format!(
                                    "scale({}) translate({}rem, {}rem)",
                                    animation_state.scale,
                                    animation_state.x,
                                    animation_state.y
                                ),
                                None => "none".into(),
                            }
                        })
                })
                .property("hasTransform", true)
        }))
        .future(card_state.found_index.signal().for_each(
            clone!(state, card_state => move |found_index| {
                if let Some(found_index) = found_index {
                    super::actions::start_animation(&state, card_state.clone(), found_index);
                }
                async {}
            }),
        ))
    })
}
