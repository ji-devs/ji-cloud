use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use crate::unwrap::UnwrapJiExt;
use shared::domain::jig::module::{ModuleId, ModuleKind};

pub const IFRAME_DATA_PARAM:&'static str = "iframe_data";

#[wasm_bindgen(inline_js = "export function is_in_iframe() { return window && window.parent && window.location !== window.parent.location; }")]
extern "C" {
    pub fn is_in_iframe() -> bool;
}

/// Init is used for bootstrapping and passing initial loaded data
#[derive(Serialize, Deserialize, Debug)]
pub struct IframeInit<T> {
    pub data: T
}

impl <T> IframeInit <T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl IframeInit <EmptyMessage> {
    pub fn empty() -> IframeInit<EmptyMessage> {
        IframeInit { data: EmptyMessage {}}
    }
}

/// Needed to avoid accidentally creating a null over the wire
/// which would be (de)serialized to anything
#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyMessage {}

impl <T: Serialize> From<IframeInit<T>> for JsValue {
    fn from(msg:IframeInit<T>) -> Self {
        (&msg).into()
    }
}

impl <T: Serialize> From<&IframeInit<T>> for JsValue {
    fn from(msg:&IframeInit<T>) -> Self {
        serde_wasm_bindgen::to_value(msg).unwrap_ji()
    }
}

impl <T: DeserializeOwned> From<JsValue> for IframeInit<T> {
    fn from(msg:JsValue) -> Self {
        serde_wasm_bindgen::from_value(msg).unwrap_ji()
    }
}

pub fn should_get_iframe_data() -> bool { 
    let url:String = dominator::routing::url().get_cloned();
    let url:web_sys::Url = web_sys::Url::new(&url).unwrap_ji();
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


/// Action is used for passing runtime messages 
#[derive(Serialize, Deserialize, Debug)]
pub struct IframeAction<T> 
{
    pub data: T
}

impl <T> IframeAction <T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl <T: Serialize> IframeAction <T> {

    pub fn try_post_message_to_parent(&self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap_ji();
        let parent = window.parent()?.unwrap_ji();

        let value = serde_wasm_bindgen::to_value(self).unwrap_ji();

        parent.post_message(&value, "*")
    }
}

impl <T: Serialize> From<IframeAction<T>> for JsValue {
    fn from(msg:IframeAction<T>) -> Self {
        (&msg).into()
    }
}

impl <T: Serialize> From<&IframeAction<T>> for JsValue {
    fn from(msg:&IframeAction<T>) -> Self {
        serde_wasm_bindgen::to_value(msg).unwrap_ji()
    }
}

impl <T: DeserializeOwned> From<JsValue> for IframeAction<T> {
    fn from(msg:JsValue) -> Self {
        serde_wasm_bindgen::from_value(msg).unwrap_ji()
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
    Start(Option<u32>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModuleToJigEditorMessage {
    AppendModule(ModuleKind, ModuleId),
    Next 
}
