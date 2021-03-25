use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::data::state::*;
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use crate::player::card::dom::CardDom;
use super::{
    game::dom::GameDom,
    ending::dom::EndingDom,
};

pub struct MainDom {
}

impl MainDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child_signal(
                state.all_cards_ended_signal()
                    .map(clone!(state => move |ended| Some({
                        if ended {
                            EndingDom::render(state.clone())
                        } else {
                            GameDom::render(state.clone())
                        }
                    })))
            )
        })
    }
}

