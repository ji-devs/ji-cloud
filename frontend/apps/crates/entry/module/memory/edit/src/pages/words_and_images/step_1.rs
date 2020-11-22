use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, dynamic_class_signal, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use std::fmt::Write;
use crate::data::*;
use itertools::Itertools;

pub struct Step1Page {
    state: Rc<BaseGameState>,
    
}

impl Step1Page {
    pub fn new(state:Rc<BaseGameState>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            state
        });

        _self
    }
 

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::words_and_images::step_1_page(), { 
        })
    }
}


