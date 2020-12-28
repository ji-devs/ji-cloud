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
use dominator::{DomBuilder, stylesheet, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id,futures::{spawn_future, AsyncLoader}};
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use components::module::page::*;
use std::pin::Pin;
use std::future::Future;
use utils::routes::{Route, DevRoute};
pub struct Page {}

impl Page {
    pub fn render() -> Rc<Self> {
        let _self = Rc::new(Page{});

        stylesheet!("A", {
            .style("text-decoration", "none")
            .style("color", "#2a2a2a")
        });

        stylesheet!("A:hover", {
            .style("text-decoration", "underline")
        });

        dominator::append_dom(&dominator::body(), render_dom());
        _self
    }
}

fn render_dom() -> Dom {
    html!("h1", {
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("margin", "10px")
        .children(vec![
            html!("a",  {
                .attribute("href", &Route::Dev(DevRoute::Showcase("001".to_string(), "grid".to_string())).to_string())
                .text("grid")
            }),
            html!("a",  {
                .attribute("href", &Route::Dev(DevRoute::Showcase("001".to_string(), "renderer".to_string())).to_string())
                .text("renderer")
            })
        ])
    })
}