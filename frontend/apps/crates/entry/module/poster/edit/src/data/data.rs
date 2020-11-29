use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::debug;
use super::raw;
use itertools::Itertools;
use std::fmt::Write;


pub struct GameState {
    pub jig_id: String,
    pub module_id: String,
    pub poster: Rc<Poster>,
}

pub struct Poster {
}

impl Poster {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set_from_raw(&self, raw_poster:raw::Poster) {
    }

}

impl GameState {
    pub fn new(jig_id:String, module_id: String) -> Self {
        Self {
            jig_id,
            module_id,
            poster: Rc::new(Poster::new()), 
        }
    }

    pub fn set_from_loaded(&self, raw_poster:raw::Poster) {
        self.poster.set_from_raw(raw_poster);
    }
}


pub struct Theme {
    pub id:&'static str,
    pub label:&'static str,
}
