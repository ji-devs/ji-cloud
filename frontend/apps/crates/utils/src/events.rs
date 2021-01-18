use dominator_helpers::{temp_make_event, make_custom_event_serde};
use super::resize::*;
use wasm_bindgen::prelude::*;

temp_make_event!(Open, "open" => web_sys::Event);
temp_make_event!(Close, "close" => web_sys::Event);


make_custom_event_serde!("module-resize", ModuleResizeEvent, ResizeInfo);


