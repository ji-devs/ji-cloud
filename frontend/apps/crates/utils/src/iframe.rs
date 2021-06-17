use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;

pub const IFRAME_DATA_PARAM:&'static str = "iframe_data";

#[derive(Serialize, Deserialize, Debug)]
pub struct IframeInit<T> {
    pub data: Option<T>
}

impl <T> IframeInit <T> {
    pub fn new(data: T) -> Self {
        Self { data: Some(data) }
    }
}

impl IframeInit <()> {
    pub fn empty() -> IframeInit<()> {
        IframeInit { data: None }
    }
}

impl <T: Serialize> From<IframeInit<T>> for JsValue {
    fn from(msg:IframeInit<T>) -> Self {
        (&msg).into()
    }
}

impl <T: Serialize> From<&IframeInit<T>> for JsValue {
    fn from(msg:&IframeInit<T>) -> Self {
        serde_wasm_bindgen::to_value(msg).unwrap_throw()
    }
}

impl <T: DeserializeOwned> From<JsValue> for IframeInit<T> {
    fn from(msg:JsValue) -> Self {
        serde_wasm_bindgen::from_value(msg).unwrap_throw()
    }
}

pub fn should_get_iframe_data() -> bool { 
    let url:String = dominator::routing::url().get_cloned();
    let url:web_sys::Url = web_sys::Url::new(&url).unwrap_throw();
    let params = url.search_params();

    match params.get(IFRAME_DATA_PARAM) {
        None => false,
        Some(value) => {
            if value == "true" {
                true
            } else {
                false
            }
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct IframeAction<T> {
    pub data: Option<T>
}

impl <T> IframeAction <T> {
    pub fn new(data: T) -> Self {
        Self { data: Some(data) }
    }
}

impl <T: Serialize> From<IframeAction<T>> for JsValue {
    fn from(msg:IframeAction<T>) -> Self {
        (&msg).into()
    }
}

impl <T: Serialize> From<&IframeAction<T>> for JsValue {
    fn from(msg:&IframeAction<T>) -> Self {
        serde_wasm_bindgen::to_value(msg).unwrap_throw()
    }
}

impl <T: DeserializeOwned> From<JsValue> for IframeAction<T> {
    fn from(msg:JsValue) -> Self {
        serde_wasm_bindgen::from_value(msg).unwrap_throw()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JigToModuleMessage {
    TimerDone,
    Play,
    Pause,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModuleToJigMessage {
    AddPoints(u32),
    StartTimer(u32),
}
