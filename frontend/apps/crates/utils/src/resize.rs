/* See https://codepen.io/dakom/pen/WNxYrQM */
/* This is slightly adapted to work on a containing element instead of window */

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CssStyleDeclaration, HtmlElement, Element};
use once_cell::sync::Lazy;
use std::sync::atomic::{Ordering, AtomicI32};
use serde::Deserialize;
use std::sync::{Mutex, MutexGuard};
use futures_signals::signal::Mutable;
use futures_signals::signal::Signal;

static RESIZE_INFO: Lazy<Mutable<ResizeInfo>> = Lazy::new(|| Mutable::new(ResizeInfo::default()));
// This event data is sent from the custom element
// And then stashed in a global for when we need it at runtime
#[derive(Deserialize, Debug, Clone, Default)]
pub struct ResizeInfo {
    pub scale: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,

    //For additional padding... maybe get rid of this...
    #[serde(rename(deserialize = "contentX"))]
    pub content_x: f64,
    #[serde(rename(deserialize = "contentY"))]
    pub content_y: f64,
    #[serde(rename(deserialize = "contentWidth"))]
    pub content_width: f64,
    #[serde(rename(deserialize = "contentHeight"))]
    pub content_height:f64 
}

pub fn set_resize_info(info:ResizeInfo) {
    RESIZE_INFO.set(info);
}

pub fn resize_info_signal() -> impl Signal<Item = ResizeInfo> {
    RESIZE_INFO.signal_cloned()
}

pub fn get_resize_info() -> ResizeInfo {
    RESIZE_INFO.get_cloned()
}


impl ResizeInfo {
    pub fn get_pos_denormalized(&self, x: f64, y: f64) -> (f64, f64) {
        (x * self.width, y * self.height)
    }

    pub fn get_pos_px(&self, x: f64, y: f64) -> (f64, f64) {

        //need to offset it by content space
        let x = x - (self.x + self.content_x);
        let y = y - (self.y + self.content_y);

        (x, y)
    }

    pub fn get_pos_normalized(&self, x: f64, y: f64) -> (f64, f64) {
        let (x, y) = self.get_pos_px(x, y);

        let x = x / self.width;
        let y = y / self.height;

        (x, y)
    }

    //"rem" really means consistent, 
    //i.e. relative to our reference artboard size
    //but we typically express that as css rem units so...
    pub fn get_pos_rem(&self, x: f64, y: f64) -> (f64, f64) {
        let (x, y) = self.get_pos_px(x, y);
        let scale = self.scale; 
        (x / scale, y / scale)
    }

    pub fn get_element_pos_rem(&self, element:&Element) -> (f64, f64) {
        let rect = element.get_bounding_client_rect();
        let elem_x = rect.left();
        let elem_y = rect.top();

        self.get_pos_rem(elem_x, elem_y)
    }

    pub fn get_element_pos_px(&self, element:&Element) -> (f64, f64) {
        //element rect is in viewport space
        let rect = element.get_bounding_client_rect();
        let elem_x = rect.left();
        let elem_y = rect.top();
     
        self.get_pos_px(elem_x, elem_y)
    }
}

