use serde::Deserialize;
use dominator_helpers::{temp_make_event, make_custom_event_serde};
use super::resize::*;
use wasm_bindgen::prelude::*;

pub use dominator::events::*;

temp_make_event!(Open, "open" => web_sys::Event);
temp_make_event!(Close, "close" => web_sys::Event);
make_custom_event_serde!("module-resize", ModuleResizeEvent, ResizeInfo);

// Custom Input
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

// Custom Toggle 
#[derive(Deserialize, Debug)]
pub struct CustomToggleData {
    pub value: bool,
}

make_custom_event_serde!("custom-toggle", CustomToggle, CustomToggleData);

impl CustomToggle {
    pub fn value(&self) -> bool {
        self.data().value
    }
}

