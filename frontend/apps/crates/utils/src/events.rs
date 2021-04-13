use serde::Deserialize;
use dominator_helpers::{temp_make_event, make_custom_event_serde, make_custom_event};
use web_sys::{File, DomRect};
use super::resize::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub use dominator::events::*;

temp_make_event!(TimeUpdate, "timeupdate" => web_sys::Event);
temp_make_event!(Ended, "ended" => web_sys::Event);

temp_make_event!(Open, "open" => web_sys::Event);
temp_make_event!(Close, "close" => web_sys::Event);

temp_make_event!(Reset, "reset" => web_sys::Event);

temp_make_event!(ExpandAll, "expand-all" => web_sys::Event);
temp_make_event!(CollapseAll, "collapse-all" => web_sys::Event);

make_custom_event_serde!("module-resize", ModuleResizeEvent, ResizeInfo);

// Custom Bounds 
#[derive(Deserialize, Debug)]
pub struct CustomBoundsData {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

make_custom_event_serde!("custom-bounds", CustomBounds, CustomBoundsData);

// Custom Change 
#[derive(Deserialize, Debug)]
pub struct CustomChangeData {
    pub value: String,
}

make_custom_event_serde!("custom-change", CustomChange, CustomChangeData);

impl CustomChange {
    pub fn value(&self) -> String {
        self.data().value
    }
}

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

// Custom Route 
#[derive(Deserialize, Debug)]
pub struct CustomRouteData {
    pub route: String,
}

make_custom_event_serde!("custom-route", CustomRoute, CustomRouteData);

impl CustomRoute {
    pub fn route(&self) -> String {
        self.data().route
    }
}

// Custom String - USE SPARINGLY, AND ONLY FOR OPAQUE STRINGS!
#[derive(Deserialize, Debug)]
pub struct CustomStringData {
    pub value: String,
}

make_custom_event_serde!("custom-string", CustomString, CustomStringData);

impl CustomString {
    pub fn value(&self) -> String {
        self.data().value
    }
}

// Custom Search 
#[derive(Deserialize, Debug)]
pub struct CustomSearchData {
    pub query: String,
}

make_custom_event_serde!("custom-search", CustomSearch, CustomSearchData);

impl CustomSearch {
    pub fn query(&self) -> String {
        self.data().query
    }
}

// Google Location 
#[derive(Deserialize, Debug)]
pub struct GoogleLocationData {
    #[serde(rename = "rawJson")]
    pub raw_json: Option<String>,
    pub input: Option<String>,

    //not going to try and decode place
}

make_custom_event_serde!("google-location", GoogleLocation, GoogleLocationData);

impl GoogleLocation {
    pub fn raw_json(&self) -> Option<String> {
        self.data().raw_json
    }
}


// #[derive(Deserialize, Debug)]
// pub struct CustomFileData {
//     pub file: File,
// }

// make_custom_event_serde!("custom-file", CustomFile, CustomFileData);

// impl CustomFile {
//     pub fn file(&self) -> File {
//         self.file().file
//     }
// }
make_custom_event!(CustomFile, "custom-file");

impl CustomFile {
    pub fn file(&self) -> web_sys::File {
        self.detail().unchecked_into()
    }
}
