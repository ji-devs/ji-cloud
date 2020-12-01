use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;
use crate::debug;
use super::raw;
use itertools::Itertools;
use std::fmt::Write;
use rand::prelude::*;
use gloo_timers::future::TimeoutFuture;
use web_sys::HtmlElement;

pub struct GameState {
    pub jig_id: String,
    pub module_id: String,
    pub loaded: Mutable<bool>,
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
            loaded: Mutable::new(false),
            poster: Rc::new(Poster::new()), 
        }
    }

    pub fn set_from_loaded(&self, raw_poster:raw::Poster) {
        self.poster.set_from_raw(raw_poster);
        self.loaded.set(true);
    }
}


pub struct Theme {
    pub id:&'static str,
    pub label:&'static str,
}

