use serde::Deserialize;
use dominator_helpers::{temp_make_event, make_custom_event_serde};
use super::resize::*;
use wasm_bindgen::prelude::*;

pub use dominator::events::*;

temp_make_event!(Open, "open" => web_sys::Event);
temp_make_event!(Close, "close" => web_sys::Event);

#[derive(Deserialize, Debug)]
pub struct CustomInputData {
    pub value: String,
}

make_custom_event_serde!("custom-input", CustomInput, CustomInputData);

impl CustomInput {
    pub fn value(&self) -> String {
        self.data().value
    }
}

make_custom_event_serde!("module-resize", ModuleResizeEvent, ResizeInfo);


