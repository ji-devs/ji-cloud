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
use dominator_helpers::{elem, with_data_id, futures::AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::shell::Shell;
use awsm_web::loaders::fetch::fetch_url;
use legacy::*;
use utils::resize::*;

pub struct StickerImage {
}

impl StickerImage {
    pub fn render(jig_id: &str, module_id: &str, img: &Image, mock: bool) -> Dom {

        html!("img-legacy", {
            .style("position", "absolute")
            .style("top", "0")
            .style("left", "0")
            .style_signal("transform", transform_signal(img.transform.clone()))
            .property("jigId", jig_id)
            .property("moduleId", module_id)
            .property("path", &format!("layers/{}", img.src))
            .property("mock", mock)
        })
    }
}

fn transform_signal(transform:[f64;6]) -> impl Signal<Item = String> {
    //TODO - adapt transform based on resize signal
    get_resize_info()
        .signal_ref(move |info| {
            format!("matrix({}, {}, {}, {}, {}, {})",
                transform[0],
                transform[1],
                transform[2],
                transform[3],
                transform[4],
                transform[5],
            )
        })
}

/*
 *
    pub scale: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[serde(rename(deserialize = "contentX"))]
    pub content_x: f64,
    #[serde(rename(deserialize = "contentY"))]
    pub content_y: f64,
    #[serde(rename(deserialize = "contentWidth"))]
    pub content_width: f64,
    #[serde(rename(deserialize = "contentHeight"))]
    pub content_height:f64 
    */
