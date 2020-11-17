use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;

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
