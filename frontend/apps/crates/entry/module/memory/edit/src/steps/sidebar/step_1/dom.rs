use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::{SignalVec, SignalVecExt},
};
use super::duplicate::dom::DuplicateDom;

pub struct Step1Dom {}
impl Step1Dom {
    pub fn render(state:Rc<State>, is_empty:bool) -> Vec<Dom> {

        let game_mode = state.game_mode.get().unwrap_ji();

        if is_empty {
            match game_mode {
                GameMode::Duplicate | GameMode::Lettering => {
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
    }
}


