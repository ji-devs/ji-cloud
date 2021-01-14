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
use dominator_helpers::{elem, with_data_id};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;

pub struct Player {
}

impl Player {
    pub fn render(jig_id: String, module_id: Option<String>) -> Dom {
        let _self = Rc::new(Self { 
        });

        html!("h1", { 
            .text("hello world") 
        })
    }
}
