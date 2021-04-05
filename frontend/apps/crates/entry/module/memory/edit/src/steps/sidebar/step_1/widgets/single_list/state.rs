use std::rc::Rc;
use wasm_bindgen::prelude::*;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use crate::data::state::GameMode;

pub struct State {
    pub mode: GameMode,
    pub list: Rc<MutableVec<Mutable<String>>>,
    pub is_placeholder: Mutable<bool>
}

type IsPlaceholder = bool;

impl State {
    pub fn new(mode: GameMode, max:usize) -> Self {
        Self {
            mode,
            list: Rc::new(MutableVec::new_with_values(
                    (0..max)
                        .map(|_| Mutable::new(String::default()))
                        .collect()
            )),
            is_placeholder: Mutable::new(true)
        }
    }

    pub fn derive_list(&self) -> Vec<String> {
        self.list
            .lock_ref()
            .iter()
            .map(|mutable_string| {
                mutable_string.get_cloned()
            })
            .filter(|x| !x.is_empty())
            .collect()
    }

}

