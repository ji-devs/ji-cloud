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
use utils::{resize::*, math::transform_2d};

pub struct StickerImage {
}

//Some properties aren't in the transform
//see https://jitap.net/media/webplayer/player.js?v=1.3.6
//
impl StickerImage {
    pub fn render(jig_id: &str, module_id: &str, img: &Image, mock: bool) -> Dom {

        html!("div", {
            .style("position", "absolute")
            .style_signal("top", top_signal(img.height)) 
            .style_signal("left", left_signal(img.width)) 
            .style_signal("width", width_signal(img.width)) 
            .style_signal("height", height_signal(img.height)) 
            .style("transform-origin", "center") 
            /*
            .style("transform", 
                format!("matrix({}, {}, {}, {}, {}, {})",
                    img.transform[0],
                    img.transform[1],
                    img.transform[2],
                    img.transform[3],
                    img.transform[4],
                    img.transform[5],
                )
            )
            */
            .child(html!("img-legacy", {
                .property("jigId", jig_id)
                .property("moduleId", module_id)
                .property("path", &format!("layers/{}", img.src))
                .property("mock", mock)
            }))
        })
    }
}

fn top_signal(height: f64) -> impl Signal<Item = String> {
    get_resize_info()
        .signal_ref(move |info| {
            //let mut transform = [ 1.0, 0.0, 0.0, 1.0, 0.0, 0.0 ];
            let ResizeInfo {scale, ..} = *info;

            //format!("{}px", (768.0 - height * scale) / 2.0)
            format!("0px")
        })
}
fn left_signal(width: f64) -> impl Signal<Item = String> {
    get_resize_info()
        .signal_ref(move |info| {
            //let mut transform = [ 1.0, 0.0, 0.0, 1.0, 0.0, 0.0 ];
            let ResizeInfo {scale, ..} = *info;

            format!("{}px", (info.width - (width * scale)) / 2.0)
        })
}

fn width_signal(width: f64) -> impl Signal<Item = String> {
    get_resize_info()
        .signal_ref(move |info| {
            //let mut transform = [ 1.0, 0.0, 0.0, 1.0, 0.0, 0.0 ];
            let ResizeInfo {scale, ..} = *info;

            format!("{}px", width * scale)
        })
}
fn height_signal(height: f64) -> impl Signal<Item = String> {
    get_resize_info()
        .signal_ref(move |info| {
            //let mut transform = [ 1.0, 0.0, 0.0, 1.0, 0.0, 0.0 ];
            let ResizeInfo {scale, ..} = *info;

            format!("{}px", height * scale)
        })
}
//TODO - not working
//backend Ji -> TT example: https://github.com/ji-devs/jitap-backend/blob/master/create/src/endpoints/playerMeta/PlayerMeta-Common.ts
//previous player example just used our format so not viable as a reference
//
fn transform_signal(transform:[f64;6], img_width: f64, img_height: f64) -> impl Signal<Item = String> {
    //TODO - adapt transform based on resize signal
    get_resize_info()
        .signal_ref(move |info| {
            //need to clone each time - otherwise changes accumulate
            let mut transform = transform.clone();

            //let mut transform = [ 1.0, 0.0, 0.0, 1.0, 0.0, 0.0 ];
            let ResizeInfo {scale, x, y, width, height, ..} = *info;

            //log::info!("{} {}", img_width, img_height);

            transform_2d::scale_mut(&mut transform, scale, scale);
            /*
            transform_2d::translate_mut(&mut transform, 
                (width / 2.0) - (img_width)/2.0,
                height / 2.0,
                //(width / 2.0) + (img_width / 2.0),
                //(height / 2.0) - (img_height / 2.0),
            );
            */
            //transform_2d::translate_mut(&mut transform, (self.x + basis[0]) * self.scale, basis[1] + self.y);

            //log::info!("{:#?}", info);

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
