use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use serde::Deserialize;
use utils::prelude::*;

pub static DUAL_LIST_CHAR_LIMIT:usize = 30;
pub static SINGLE_LIST_CHAR_LIMIT:usize = 30;
pub static CARD_TEXT_LIMIT_WIDTH:f64 = 150.0;
pub static CARD_TEXT_LIMIT_HEIGHT:f64 = 150.0;

macro_rules! config_path {
    ($e:tt) => { 
        concat!("../../../../../../../config/", $e)
    } 
}
static EDITOR_CONFIG:OnceCell<EditorConfig> = OnceCell::new();

#[derive(Deserialize)]
struct EditorConfig {
}

pub fn init() {
    EDITOR_CONFIG.set(serde_json::from_str(include_str!(config_path!("module/poster/editor.json"))).unwrap_ji());
}

