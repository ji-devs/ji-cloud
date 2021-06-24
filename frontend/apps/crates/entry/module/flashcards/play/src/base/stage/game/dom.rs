use dominator::{html, Dom, clone};
use std::rc::Rc;
use futures_signals::{
    signal::SignalExt,
    signal_vec::SignalVecExt
};
use utils::prelude::*;
use crate::base::{
    state::*,
    card::{
        state::*,
        dom::render_media as render_card_media
    }
};

pub fn render(state: Rc<Base>) -> Dom {

    html!("play-main", {
        .property("nCards", state.cards.len() as f64)
        .children(
            //Always render the cards so they take the grid spots
            //"hiding" is via `visiblity`, not `display`
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
        .property_signal("flipped", card.is_flipped(&state))
        .property("theme", state.theme_id.as_str_id())
        .property("mode", state.mode.as_str_id())
        .property("side", card.side.as_str())
        .style_signal("visibility", card.is_found().map(|flag| {
            if flag {
                "hidden"
            } else {
                "visible"
            }
        }))
        .child(render_card_media(&card, state.mode, state.theme_id))
        .event(clone!(state, card_id => move |evt:events::Click| {
            if let Some((id_1, id_2)) = super::actions::card_click(state.clone(), card_id) {
                super::actions::evaluate(state.clone(), id_1, id_2);
            }
        }))
        .after_inserted(clone!(card => move |elem| {
            *card.main_elem.borrow_mut() = Some(elem);
        }))
        
        
    })
}

