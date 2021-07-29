use dominator::{html, Dom, clone};
use std::rc::Rc;
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use super::{
    game::dom::render as render_game,
    ending::dom::render as render_ending,
};

use crate::base::state::*;

pub fn render(state: Rc<Base>) -> Dom {
    html!("empty-fragment", {
        .property("slot", "main")
        .child_signal(
            state.all_cards_ended_signal()
                .map(clone!(state => move |ended| Some({
                    if ended {
                        render_ending(state.clone())
                    } else {
                        render_game(state.clone())
                    }
                })))
        )
    })
}
