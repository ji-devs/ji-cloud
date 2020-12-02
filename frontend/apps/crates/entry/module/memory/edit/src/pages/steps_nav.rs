use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, ReadOnlyMutable},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::debug;
use utils::components::module_page::*;
use async_trait::async_trait;
use super::choose_mode;

pub fn apply_steps_nav(dom:DomBuilder<HtmlElement>, state: Rc<State>) -> DomBuilder<HtmlElement> {
    apply_methods!(dom, {
        .with_data_id!("top-step-1", {
            .event(clone!(state => move |evt:events::Click| {
                state.step.set(Step::One);
            }))
        })
        .with_data_id!("top-step-2", {
            .event(clone!(state => move |evt:events::Click| {
                if state.content_mode.get() != ContentMode::TextInit {
                    state.step.set(Step::Two);
                }
            }))
        })
        .with_data_id!("top-step-3", {
            .event(clone!(state => move |evt:events::Click| {
                if state.content_mode.get() != ContentMode::TextInit {
                    state.step.set(Step::Three);
                }
            }))
        })
        .with_data_id!("top-step-4", {
            .event(clone!(state => move |evt:events::Click| {
                if state.content_mode.get() != ContentMode::TextInit {
                    state.step.set(Step::Four);
                }
            }))
        })
    })
}
