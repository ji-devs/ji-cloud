use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, always},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::templates;
use utils::components::module_page::*;
use std::pin::Pin;

pub fn render() -> Dom {
    html!("div", {.text("hello")})
}
