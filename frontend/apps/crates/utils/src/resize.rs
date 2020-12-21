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

static RESIZE_INFO: Lazy<Mutex<ResizeInfo>> = Lazy::new(|| Mutex::new(ResizeInfo::default()));
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
    *RESIZE_INFO.lock().unwrap_throw() = info;
}

//To borrow the lock for a bunch of reads
pub fn lock_resize_info() -> MutexGuard<'static, ResizeInfo> {
    RESIZE_INFO.lock().unwrap_throw()
}

pub fn map_resize_info<F, A>(map: F) -> A
where
    F: FnOnce(&ResizeInfo) -> A
{
    map(&lock_resize_info())
}
