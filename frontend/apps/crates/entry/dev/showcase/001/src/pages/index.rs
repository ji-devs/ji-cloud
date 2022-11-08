use awsm_web::dom::*;
use components::module::page::*;
use dominator::{apply_methods, clone, events, html, stylesheet, Dom, DomBuilder};
use dominator_helpers::{
    elem,
    futures::{spawn_future, AsyncLoader},
    with_data_id,
};
use futures::future::ready;
use futures_signals::{
    map_ref,
    signal::{always, Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle,
};
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use utils::routes::{DevRoute, Route};
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
use web_sys::{Element, HtmlElement, HtmlInputElement};
pub struct Page {}

impl Page {
    pub fn render() -> Rc<Self> {
        let _self = Rc::new(Page {});

        stylesheet!("A", {
            .style("text-decoration", "none")
            .style("color", "#2a2a2a")
        });

        stylesheet!("A:hover", {
            .style("text-decoration", "underline")
        });

        dominator::append_dom(&dominator::get_id("root"), render_dom());
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
                .attr("href", &Route::Dev(DevRoute::Showcase("001".to_string(), "grid".to_string())).to_string())
                .text("grid")
            }),
            html!("a",  {
                .attr("href", &Route::Dev(DevRoute::Showcase("001".to_string(), "renderer".to_string())).to_string())
                .text("renderer")
            })
        ])
    })
}
