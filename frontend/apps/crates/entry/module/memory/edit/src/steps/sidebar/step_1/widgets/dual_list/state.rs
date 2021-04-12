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
    pub left: Rc<MutableVec<Mutable<String>>>,
    pub right: Rc<MutableVec<Mutable<String>>>,
    pub is_placeholder: Mutable<bool>
}

type IsPlaceholder = bool;

impl State {
    pub fn new(app: Rc<AppState>, max:usize) -> Self {
        Self {
            app,
            left: Rc::new(MutableVec::new_with_values(
                    (0..max)
                        .map(|_| Mutable::new(String::default()))
                        .collect()
            )),
            right: Rc::new(MutableVec::new_with_values(
                    (0..max)
                        .map(|_| Mutable::new(String::default()))
                        .collect()
            )),
            is_placeholder: Mutable::new(true)
        }
    }

    pub fn derive_list(&self) -> Vec<(String, String)> {
        self.left
            .lock_ref()
            .iter()
            .map(|mutable_string| {
                mutable_string.get_cloned()
            })
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .zip(
                self.right
                    .lock_ref()
                    .iter()
                    .map(|mutable_string| {
                        mutable_string.get_cloned()
                    })
                    .filter(|x| !x.is_empty())
                    .map(|x| x.to_string())
            )
            .collect()
    }
    pub fn clear(&self) {
        for mutable_string in self.left.lock_ref().iter() {
            mutable_string.set(String::default());
        }
        for mutable_string in self.right.lock_ref().iter() {
            mutable_string.set(String::default());
        }

        self.is_placeholder.set_neq(true);
    }

}

