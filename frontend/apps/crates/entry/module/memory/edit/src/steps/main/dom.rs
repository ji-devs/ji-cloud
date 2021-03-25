use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::events;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
use super::pair::dom::PairDom;

pub struct MainDom {}
impl MainDom {
    pub fn render(state:Rc<State>) -> Dom {

        let game_mode = state.game_mode.get().unwrap_throw();

        html!("empty-fragment", {
            .property("slot", "main")
            .child_signal(state.is_empty_signal().map(clone!(state => move |is_empty| {
                Some(
                    if is_empty {
                        html!("module-main-empty")
                    } else {
                        html!("main-cards", {
                            .children_signal_vec({
                                state.step.signal()
                                    .switch_signal_vec(clone!(state => move |step| {
                                        state.pairs
                                            .signal_vec_cloned()
                                            .enumerate()
                                            .map(clone!(state => move |(index, pair)| {
                                                PairDom::render(state.clone(), game_mode, step, index, pair)
                                            }))
                                    }))
                            })
                        })
                    }
                )
            })))
        })
    }
}
