use crate::base::{card::state::*, state::*};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

use components::module::_groups::cards::play::card::dom::{Size, render_card_mixin, CardOptions};

pub fn render(state: Rc<Base>) -> Dom {
    html!("play-main", {
        .property("nCards", state.cards.len() as f64)
        .children(
            //Always render the cards so they take the grid spots
            //"hiding" is via `visiblity`, not `display`
            state.cards
                .iter()
                .map(clone!(state => move |card| {
                    render_main_card(state.clone(), card.clone())
                }))
        )
    })
}

fn render_main_card(state: Rc<Base>, card_state: Rc<CardState>) -> Dom {
    let card_id = &card_state.id;

    let options = CardOptions::new(
        &card_state.card,
        state.theme_id,
        state.mode,
        card_state.side,
        Size::Memory,
    );

    render_card_mixin(options, |dom| {
        dom
            .style_signal("visibility", card_state.is_found().map(|found| if found { "hidden" } else { "visible" }))
            .property_signal("flipped", card_state.is_flipped(&state))
            .event(clone!(state, card_id => move |_evt:events::Click| {
                if let Some((id_1, id_2)) = super::actions::card_click(state.clone(), card_id) {
                    super::actions::evaluate(state.clone(), id_1, id_2);
                }
            }))
            .after_inserted(clone!(card_state => move |elem| {
                *card_state.main_elem.borrow_mut() = Some(elem);
            }))
    })
}
