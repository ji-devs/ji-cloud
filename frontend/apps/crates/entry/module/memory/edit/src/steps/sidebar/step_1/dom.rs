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

        let mode = state.mode.get().unwrap_ji();

        match mode {
            Mode::WordsAndImages => {
                WordsAndImagesDom::render(state.clone(), is_empty)
            },
            _ => {
                let is_dual = {
                    match mode {
                        Mode::Duplicate | Mode::Lettering => false,
                        _ => true
                    }
                };
                WordsDom::render(state.clone(), is_empty, is_dual)
            }
        }
    }
}


