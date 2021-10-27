use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::unwrap::UnwrapJiExt;
use shared::domain::jig::{module::ModuleId, LiteModule};

pub const IFRAME_DATA_PARAM: &str = "iframe_data";

#[wasm_bindgen(
    inline_js = "export function is_in_iframe() { return window && window.parent && window.location !== window.parent.location; }"
)]
extern "C" {
    pub fn is_in_iframe() -> bool;
}

pub trait IframeMessageExt {
    fn try_post_message_to_top<'a>(&'a self) -> Result<(), JsValue>
    where
        &'a Self: Into<JsValue>,
    {
        let window = web_sys::window().unwrap_ji();
        let top = window.top()?.unwrap_ji();

        top.post_message(&self.into(), "*")
    }
    fn try_post_message_to_parent<'a>(&'a self) -> Result<(), JsValue>
    where
        &'a Self: Into<JsValue>,
    {
        let window = web_sys::window().unwrap_ji();
        let parent = window.parent()?.unwrap_ji();

        parent.post_message(&self.into(), "*")
    }
}

impl<T: Serialize> IframeMessageExt for IframeInit<T> {}
impl<T: Serialize> IframeMessageExt for IframeAction<T> {}

/// Init is used for bootstrapping and passing initial loaded data
#[derive(Serialize, Deserialize, Debug)]
pub struct IframeInit<T> {
    pub data: T,
}

impl<T> IframeInit<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl IframeInit<EmptyMessage> {
    pub fn empty() -> IframeInit<EmptyMessage> {
        IframeInit {
            data: EmptyMessage {},
        }
    }
}

/// Needed to avoid accidentally creating a null over the wire
/// which would be (de)serialized to anything
#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyMessage {}

impl<T: Serialize> From<IframeInit<T>> for JsValue {
    fn from(msg: IframeInit<T>) -> Self {
        (&msg).into()
    }
}

impl<T: Serialize> From<&IframeInit<T>> for JsValue {
    fn from(msg: &IframeInit<T>) -> Self {
        serde_wasm_bindgen::to_value(msg).unwrap_ji()
    }
}

impl<T: DeserializeOwned> From<JsValue> for IframeInit<T> {
    fn from(msg: JsValue) -> Self {
        serde_wasm_bindgen::from_value(msg).unwrap_ji()
    }
}

pub fn should_get_iframe_data() -> bool {
    crate::routes::is_param_bool(IFRAME_DATA_PARAM)
}

/// Action is used for passing runtime messages
#[derive(Serialize, Deserialize, Debug)]
pub struct IframeAction<T> {
    pub data: T,
}

impl<T> IframeAction<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

#[wasm_bindgen(inline_js = "export function temp_log(val) { console.log(val); }")]
extern "C" {
    fn temp_log(val: &JsValue);
}

impl<T: Serialize> From<IframeAction<T>> for JsValue {
    fn from(msg: IframeAction<T>) -> Self {
        (&msg).into()
    }
}

impl<T: Serialize> From<&IframeAction<T>> for JsValue {
    fn from(msg: &IframeAction<T>) -> Self {
        serde_wasm_bindgen::to_value(msg).unwrap_ji()
    }
}

impl<T: DeserializeOwned> From<JsValue> for IframeAction<T> {
    fn from(msg: JsValue) -> Self {
        serde_wasm_bindgen::from_value(msg).unwrap_ji()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JigToModulePlayerMessage {
    TimerDone,
    Play,
    Pause,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModuleToJigPlayerMessage {
    AddPoints(u32),
    Start(Option<u32>),
    Stop,
    Next,
    JumpToIndex(usize),
    JumpToId(ModuleId),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModuleToJigEditorMessage {
    AppendModule(LiteModule),
    Next,
}
