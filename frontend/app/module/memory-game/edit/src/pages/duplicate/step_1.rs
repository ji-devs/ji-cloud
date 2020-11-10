use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;

pub struct Step1Page {
}

impl Step1Page {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
        });

        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::duplicate::step_1_page(), { })
    }
}
