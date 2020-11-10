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
use super::step_1::Step1Page;
use crate::data::*;
use crate::debug;

pub struct DuplicatePage {
    pub step: Mutable<Step>
}

impl DuplicatePage {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            step: Mutable::new(debug::settings().step.unwrap_or(Step::One)),
        });

        _self
    }
    
    fn dom_signal(_self:Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        _self.step.signal_ref(clone!(_self => move |step| {
            match step {
                Step::One => Some(Step1Page::render(Step1Page::new())),
                _ => None,
            }

        }))
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", { .child_signal(Self::dom_signal(_self.clone())) } )
    }
}
