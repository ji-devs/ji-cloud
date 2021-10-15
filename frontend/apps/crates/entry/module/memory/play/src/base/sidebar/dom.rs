use crate::base::{card::state::*, state::*};
use dominator::{clone, html, Dom};
use futures_signals::signal::{always, SignalExt};
use std::rc::Rc;

use components::module::_groups::cards::play::card::dom::{
    render_dynamic_card_mixin, DynamicCardOptions, SimpleTransform, Size,
};

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

    let flipped_signal = always(true);
    let transparent_signal = always(false);
    let hidden_signal = card_state.is_found().map(|found| !found);
    let get_simple_transform = clone!(card_state => move || {
        card_state.animation_state_signal()
            .map(|animation_state| {
                animation_state.map(|animation_state| {
                    SimpleTransform {
                        x: animation_state.x,
                        y: animation_state.y,
                        scale: animation_state.scale,
                    }
                })
            })
    });

    let options = DynamicCardOptions::new(
        card,
        theme_id,
        mode,
        side,
        size,
        flipped_signal,
        transparent_signal,
        hidden_signal,
        Some(get_simple_transform),
    );

    render_dynamic_card_mixin(options, |dom| {
        dom.future(card_state.found_index.signal().for_each(
            clone!(state, card_state => move |found_index| {
                if let Some(found_index) = found_index {
                    super::actions::start_animation(&state, card_state.clone(), found_index);
                }
                async {}
            }),
        ))
    })
}
