use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::events;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::{SignalVec, SignalVecExt},
};
use super::duplicate::dom::DuplicateDom;

pub struct Step1Dom {}
impl Step1Dom {
    pub fn render(state:Rc<State>) -> impl SignalVec<Item = Dom> {

        let game_mode = state.game_mode.get().unwrap_throw();

        state.is_empty_signal()
            .map(clone!(state => move |is_empty| {
                if is_empty {
                    match game_mode {
                        GameMode::Duplicate => {
                            DuplicateDom::render(state.clone())
                        },
                        _ => {
                            Vec::new()
                        }
                    }
                } else {
                    vec![
                        html!("step1-sidebar-empty", {
                            .property("slot", "content")
                        })
                    ]
                }
            }))
            .to_signal_vec()
        /*
        state.pairs.signal_vec_cloned()
            .to_signal_cloned()
            .map(clone!(state => move |pairs| {
                //let mut children:Vec<Dom> = Vec::new();

                if pairs.is_empty() {
                    match game_mode {
                        GameMode::Duplicate => {
                            DuplicateDom::render(state.clone())
                        },
                        _ => {
                            Vec::new()
                        }
                    }
                } else {
                    vec![
                        html!("step1-sidebar-empty", {
                            .property("slot", "content")
                        })
                    ]
                }
            }))
            .to_signal_vec()
            */
    }
}


