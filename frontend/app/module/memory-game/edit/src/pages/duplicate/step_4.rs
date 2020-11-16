use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, spawn_future, dynamic_class_signal, AsyncLoader};
use crate::utils::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use std::fmt::Write;
use crate::data::*;
use itertools::Itertools;
use crate::config;

pub struct Step4Page {
    state: Rc<DuplicateState>,
}

impl Step4Page {
    pub fn new(state:Rc<DuplicateState>) -> Rc<Self> {

        let _self = Rc::new(Self { 
            state,
        });

        _self
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::duplicate::step_4_page(), { 
            .with_data_id!("top-step-1", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.state.step.set(Step::One);
                }))
            })
            .with_data_id!("top-step-2", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.state.step.set(Step::Two);
                }))
            })
            /*
            .with_data_id!("next", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.state.step.set(Step::Three);
                }))
            })
            */
        })
    }
}


