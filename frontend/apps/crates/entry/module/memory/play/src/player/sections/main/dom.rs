use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::data::state::*;
use futures_signals::signal_vec::SignalVecExt;
use crate::player::card::dom::CardDom;

pub struct MainDom {
}

impl MainDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("play-main", {
            .property("slot", "main")
            .property("nCards", state.cards.len() as f64)
            .children(
                state.cards
                    .iter()
                    .map(clone!(state => move |card| {
                        CardDom::render_main(state.clone(), card.clone())
                    }))
            )

        })
    }
}

