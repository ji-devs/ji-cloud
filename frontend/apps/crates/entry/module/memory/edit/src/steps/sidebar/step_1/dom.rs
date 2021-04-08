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
use super::{
    duplicate::dom::DuplicateDom,
    words_images::dom::WordsAndImagesDom,
};


pub struct Step1Dom {}
impl Step1Dom {
    pub fn render(state:Rc<State>, is_empty:bool) -> Dom {

        let game_mode = state.game_mode.get().unwrap_ji();

        if is_empty {
            match game_mode {
                GameMode::Duplicate | GameMode::Lettering => {
                    DuplicateDom::render(state.clone())
                },
                GameMode::WordsAndImages => {
                    WordsAndImagesDom::render(state.clone())
                },
                _ => {
                    html!("empty-fragment")
                }
            }
        } else {
            html!("step1-sidebar-empty", {
                .property("slot", "content")
            })
        }
    }
}


