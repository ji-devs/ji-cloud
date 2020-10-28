use wasm_bindgen::prelude::*;
use js_sys::Promise;
use core::settings::Settings;
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;

#[wasm_bindgen(module = "/js/google-maps.js")]
extern "C" {
    pub fn bind_google_maps(elem: HtmlInputElement, on_change: &Closure<dyn FnMut(String)>);
    pub fn geolocate();
}
