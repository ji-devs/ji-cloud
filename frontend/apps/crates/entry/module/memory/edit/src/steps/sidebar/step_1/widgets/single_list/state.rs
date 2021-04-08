use std::rc::Rc;
use wasm_bindgen::prelude::*;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use crate::data::state::{State as AppState, GameMode};

pub struct State {
    pub app: Rc<AppState>,
    pub list: Rc<MutableVec<Mutable<String>>>,
    pub is_placeholder: Mutable<bool>
}

type IsPlaceholder = bool;

impl State {
    pub fn new(app: Rc<AppState>, max:usize) -> Self {
        Self {
            app,
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

