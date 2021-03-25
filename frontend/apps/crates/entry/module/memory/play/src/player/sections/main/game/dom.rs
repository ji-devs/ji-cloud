use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::data::state::*;
use futures_signals::signal_vec::SignalVecExt;
use crate::player::card::dom::CardDom;

pub struct GameDom {
}

impl GameDom {
    pub fn render(state: Rc<State>) -> Dom {

        html!("play-main", {
            .property("nCards", state.cards.len() as f64)
            .children(
                //Always render the cards so they take the grid spots
                //"hiding" is via `visiblity`, not `display`
                state.cards
                    .iter()
                    .map(clone!(state => move |card| {
                        CardDom::render_main(state.clone(), card.clone())
                    }))
            )
        })
    }
}

