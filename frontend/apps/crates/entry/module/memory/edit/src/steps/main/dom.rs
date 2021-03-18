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
use super::card_pair::dom::PairDom;

pub struct MainDom {}
impl MainDom {
    pub fn render(state:Rc<State>) -> Dom {

        let game_mode = state.game_mode.get().unwrap_throw();

        html!("empty-fragment", {
            //Top-level future listeners here affect other areas too
            //it's just kept on main because it needs to be somewhere
            //main makes sense and I'd prefer to not pollute index
            .future(state.pairs.signal_vec_cloned().for_each(clone!(state => move |pairs| {
                state.is_empty.set_neq(state.pairs.lock_ref().is_empty());
                async {}
            })))
            .property("slot", "main")
            .child_signal(state.is_empty.signal().map(clone!(state => move |is_empty| {
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
