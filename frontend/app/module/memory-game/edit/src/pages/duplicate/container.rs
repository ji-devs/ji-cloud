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
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use super::step_1::Step1Page;
use super::step_2::Step2Page;
use crate::data::*;
use crate::debug;



pub struct DuplicatePage {
    pub state: Rc<DuplicateState>
}

impl DuplicatePage {
    pub fn new(state: Rc<DuplicateState>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            state
        });

        _self
    }

    fn dom_signal(_self:Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        _self.state.step.signal_ref(clone!(_self => move |step| {
            match step {
                Step::One => Some(Step1Page::render(Step1Page::new(_self.state.clone()))),
                Step::Two => Some(Step2Page::render(Step2Page::new(_self.state.clone()))),
                _ => None,
            }

        }))
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", { 
            .child_signal(Self::dom_signal(_self.clone())) 
        } )
    }
}
