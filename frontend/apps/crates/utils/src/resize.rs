/* See https://codepen.io/dakom/pen/WNxYrQM */
/* This is slightly adapted to work on a containing element instead of window */

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CssStyleDeclaration, HtmlElement, Element};
use config::{STAGE_WIDTH, STAGE_HEIGHT, STAGE_PADDING_X_PERC, STAGE_PADDING_Y_PERC, STAGE_RATIO};
use once_cell::sync::Lazy;
use std::sync::atomic::{Ordering, AtomicI32};
use serde::Deserialize;
use std::sync::{Mutex, MutexGuard};
use futures_signals::signal::Mutable;

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

pub fn get_resize_info() -> Mutable<ResizeInfo> {
    RESIZE_INFO.clone()
}
