use std::rc::Rc;
use wasm_bindgen::prelude::*;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};

pub struct State {
    pub list: Rc<MutableVec<Mutable<String>>>,
    pub is_placeholder: Mutable<bool>
}

type IsPlaceholder = bool;

impl State {
    pub fn new(list: Rc<MutableVec<Mutable<String>>>) -> Self {
        Self {
            list,
            is_placeholder: Mutable::new(true)
        }
    }


}

