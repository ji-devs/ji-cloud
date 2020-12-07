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
use dominator_helpers::{elem, with_data_id};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;

    
pub fn render(state: Rc<State>) -> Dom {
    elem!(templates::choose_mode_page(), { 
        .with_data_id!("btn-duplicate", {
            .event(clone!(state => move |evt:events::Click| {
                state.change_mode(GameMode::Duplicate);
            }))
        })
        .with_data_id!("btn-words-and-images", {
            .event(clone!(state => move |evt:events::Click| {
                state.change_mode(GameMode::WordsAndImages);
            }))
        })
        .with_data_id!("btn-begins-with", {
            .event(clone!(state => move |evt:events::Click| {
                //log::info!("begins-with clicked!");
            }))
        })
        .with_data_id!("btn-lettering", {
            .event(clone!(state => move |evt:events::Click| {
                //log::info!("lettering clicked!");
            }))
        })
    })
}
