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

pub struct State {
    pub jig_id: String,
    pub module_id: String,
}

impl State {
    pub fn new(jig_id:String, module_id: String, raw_poster:raw::Poster) -> Rc<Self> {
        Rc::new(Self {
            jig_id,
            module_id,
        })
    }
}


pub struct Theme {
    pub id:&'static str,
    pub label:&'static str,
}

