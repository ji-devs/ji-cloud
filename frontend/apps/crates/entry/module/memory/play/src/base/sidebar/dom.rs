use crate::base::{card::state::*, state::*};
use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use std::rc::Rc;

use components::module::_groups::cards::play::card::dom::{render_card_mixin, CardOptions, Size};

pub fn render(state: Rc<Base>) -> Dom {
    html!("play-sidebar", {
        .prop("slot", "sidebar")
        .prop_signal("animating", state.is_animating.signal().map(|count| count > 0))
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

    let animation_state_signal = map_ref! {
        let found_pairs = state.found_pairs.signal_cloned(),
        let animation_state = card_state.animation_state_signal().map(|s| s.clone())
            => {
                (found_pairs.len(), animation_state.clone())
        }
    };

    render_card_mixin(options, |dom| {
        dom.style_signal(
            "display",
            card_state
                .is_found()
                .map(|found| if !found { "none" } else { "block" }),
        )
        .prop("flipped", true)
        .apply(clone!(card_state => move |dom| {
            dom
                .style_signal("transform", {
                    animation_state_signal
                        .map(move |(mut found_pairs, animation_state)| {
                            match animation_state {
                                Some(animation_state) => {
                                    if found_pairs > 0 {
                                        found_pairs -= 1;
                                    }

                                    let mut y_offset = 0.0;
                                    if animation_state.finished {
                                        let found_index = card_state.found_index.get().unwrap_or(0);
                                        let offset = found_pairs - found_index;
                                        y_offset = offset as f64 * 222.0;
                                    }
                                    format!(
                                        "scale({}) translate({}rem, {}rem)",
                                        animation_state.scale,
                                        animation_state.x,
                                        animation_state.y + y_offset,
                                    )
                                },
                                None => "none".into(),
                            }
                        })
                })
                .prop("hasTransform", true)
        }))
        .future(card_state.found_index.signal().for_each(
            clone!(state, card_state => move |found_index| {
                if found_index.is_some() {
                    super::actions::start_animation(&state, card_state.clone());
                }
                async {}
            }),
        ))
        .future(
            card_state
                .animation_state_signal()
                .map(|state| state.map(|state| state.finished))
                .for_each(clone!(state => move |finished| {
                    if let Some(finished) = finished {
                        if finished {
                            state.is_animating.set(state.is_animating.get() - 1);
                        }
                    }

                    async {}
                })),
        )
    })
}
