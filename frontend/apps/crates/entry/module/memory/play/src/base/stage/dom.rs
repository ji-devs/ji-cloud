use dominator::{html, Dom, clone};
use std::rc::Rc;
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use components::module::_common::play::prelude::*;
use super::game::dom::render as render_game;

use crate::base::state::*;

pub fn render(state: Rc<Base>) -> Dom {
    html!("empty-fragment", {
        .future(state.all_cards_ended_signal().dedupe().for_each(clone!(state => move |ended| {
            if ended {
                state.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Positive)));
            }
            async {}
        })))
        .property("slot", "main")
        .child_signal(
            state.all_cards_ended_signal()
                .map(clone!(state => move |ended| { 
                    if ended {
                        None 
                    } else {
                        Some(render_game(state.clone()))
                    }
                }))
        )
    })
}
