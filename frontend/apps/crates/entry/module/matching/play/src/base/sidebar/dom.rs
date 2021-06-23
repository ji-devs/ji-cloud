use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::base::{
    state::*,
    card::{
        state::*,
        dom::render_media as render_card_media
    }
};
use futures_signals::{
    signal::SignalExt,
    signal_vec::SignalVecExt
};
use utils::prelude::*;

pub fn render(state: Rc<Base>) -> Dom {
    html!("play-sidebar", {
        .property("slot", "sidebar")
        .children(

            //It's simpler to just always render but hide via CSS until we start the animation
            state.cards
                .iter()
                .map(clone!(state => move |card| {
                    render_card(state.clone(), card.clone())
                }))
        )
    })
}

fn render_card(state: Rc<Base>, card: Rc<CardState>) -> Dom {
    let card_id = &card.id;

    html!("play-card", {
        .future(card.found_index.signal().for_each(clone!(state, card => move |found_index| {
            if let Some(found_index) = found_index {
                super::actions::start_animation(&state, card.clone(), found_index);
            }
            async {}
        })))
        .property("side", card.side.as_str())
        .style_signal("display", card.is_found().map(|flag| {
            if flag {
                "block"
            } else {
                "none"
            }
        }))
        .property_signal("translateX", {
            card.transform_signal().map(|t| match t {
                Some(t) => t.x,
                None => 0.0
            })
        }) 
        .property_signal("translateY", {
            card.transform_signal().map(|t| match t {
                Some(t) => t.y,
                None => 0.0
            })
        }) 
        .property_signal("scale", {
            card.transform_signal().map(|t| match t {
                Some(t) => t.scale,
                None => 1.0 
            })
        }) 
        .property("flipped", true) 
        .property("theme", state.theme_id.as_str_id())
        .property("transform", true)
        .child(render_card_media(&card, state.mode, state.theme_id))
    })
}
