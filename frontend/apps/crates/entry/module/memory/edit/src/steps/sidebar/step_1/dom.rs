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
    words::dom::WordsDom,
    words_images::dom::WordsAndImagesDom,
};


pub struct Step1Dom {}
impl Step1Dom {
    pub fn render(state:Rc<State>, is_empty:bool) -> Dom {

        let game_mode = state.game_mode.get().unwrap_ji();

        match game_mode {
            GameMode::WordsAndImages => {
                WordsAndImagesDom::render(state.clone(), is_empty)
            },
            _ => {
                let is_dual = {
                    match game_mode {
                        GameMode::Duplicate | GameMode::Lettering => false,
                        _ => true
                    }
                };
                WordsDom::render(state.clone(), is_empty, is_dual)
            }
        }
    }
}


