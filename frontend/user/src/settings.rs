use wasm_bindgen::prelude::*;
use std::fmt;
use cfg_if::cfg_if;
use lazy_static::lazy_static;
use strum_macros::{Display, EnumString};
use serde::{Serialize};

lazy_static! {
    pub static ref SETTINGS:Settings = Settings::new();
}


#[derive(Serialize)]
pub struct Settings {
    pub api_target: ApiTarget,
    pub api_url_base: &'static str,
    pub api_js_target: ApiTarget,
    pub api_js_url_base: &'static str,
    pub media_url_base: &'static str,
    pub host_url_base: Option<&'static str>,
    pub firebase_dev: bool,
}

impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "api_target is [{}] and api_js_target is [{}]", self.api_target, self.api_js_target)
    }
}

#[derive(Serialize, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ApiTarget {
    Local,
    Sandbox,
    Release,
}

impl Settings {
    pub fn new_local() -> Self {
        Self {
            api_target: ApiTarget::Local,
            api_url_base: "http://localhost:8081",
            api_js_target: ApiTarget::Local,
            api_js_url_base: "http://localhost:8082",
            media_url_base: "http://localhost:4102",
            host_url_base: None,
            firebase_dev: true,
        }
    }
    pub fn new_sandbox() -> Self {
        Self {
            api_target: ApiTarget::Sandbox,
            api_url_base: "https://sandbox.api.jicloud.org",
            api_js_target: ApiTarget::Sandbox,
            api_js_url_base: "https://sandbox.api-js.jicloud.org",
            media_url_base: "https://storage.googleapis.com/ji-cloud-eu",
            host_url_base: None,
            firebase_dev: true,
        }
    }
    pub fn new_release() -> Self {
        Self {
            api_target: ApiTarget::Release,
            api_url_base: "https://api.jicloud.org",
            api_js_target: ApiTarget::Release,
            api_js_url_base: "https://api-js.jicloud.org",
            media_url_base: "https://storage.googleapis.com/ji-cloud-eu",
            host_url_base: None,
            firebase_dev: false,
        }
    }
    
    cfg_if! {
        if #[cfg(feature = "local")] {
            pub fn new() -> Self { Self::new_local() }
        } else if #[cfg(feature = "sandbox")] {
            pub fn new() -> Self { Self::new_sandbox() }
        } else if #[cfg(feature = "release")] {
            pub fn new() -> Self { Self::new_release() }
        } else {
            pub fn new() -> Self { unimplemented!() }
        } 
    }
}


#[wasm_bindgen]
pub fn get_settings() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&*SETTINGS).map_err(|err| err.into())
}