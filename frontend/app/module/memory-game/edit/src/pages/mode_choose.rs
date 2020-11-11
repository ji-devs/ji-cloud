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
use crate::data::*;

pub struct ModeChoosePage <F: Fn(GameMode) + 'static> {
    pub on_change: F 
}

impl <F: Fn(GameMode) + 'static> ModeChoosePage<F> {
    pub fn new(on_change: F) -> Rc<Self> {
        let _self = Rc::new(Self { 
            on_change
        });

        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::mode_choose_page(), { 
            .with_data_id!("btn-duplicate", {
                .event(clone!(_self => move |evt:events::Click| {
                    (_self.on_change)(GameMode::Duplicate);
                }))
            })
            .with_data_id!("btn-words-and-images", {
                .event(clone!(_self => move |evt:events::Click| {
                    //log::info!("words-and-images clicked!");
                }))
            })
            .with_data_id!("btn-begins-with", {
                .event(clone!(_self => move |evt:events::Click| {
                    //log::info!("begins-with clicked!");
                }))
            })
            .with_data_id!("btn-lettering", {
                .event(clone!(_self => move |evt:events::Click| {
                    //log::info!("lettering clicked!");
                }))
            })
        })
    }
}
