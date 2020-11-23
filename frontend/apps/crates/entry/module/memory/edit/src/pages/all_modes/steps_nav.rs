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
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::config;

pub fn apply_steps_nav(dom:DomBuilder<HtmlElement>, state: Rc<BaseGameState>) -> DomBuilder<HtmlElement> {
    apply_methods!(dom, {
        .with_data_id!("top-step-1", {
            .event(clone!(state => move |evt:events::Click| {
                state.step.set(Step::One);
            }))
        })
        .with_data_id!("top-step-2", {
            .event(clone!(state => move |evt:events::Click| {
                state.step.set(Step::Two);
            }))
        })
        .with_data_id!("top-step-4", {
            .event(clone!(state => move |evt:events::Click| {
                state.step.set(Step::Four);
            }))
        })
    })
}
