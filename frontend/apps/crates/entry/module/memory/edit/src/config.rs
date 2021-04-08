use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use serde::Deserialize;
use utils::prelude::*;
use crate::data::state::GameMode;

macro_rules! config_path {
    ($e:tt) => { 
        concat!("../../../../../../../config/", $e)
    } 
}
static EDITOR_CONFIG:OnceCell<EditorConfig> = OnceCell::new();

#[derive(Deserialize)]
struct EditorConfig {
    init: InitConfig 
}
#[derive(Deserialize)]
struct InitConfig {
    duplicate: Vec<String> 
}

pub fn init() {
    EDITOR_CONFIG.set(serde_json::from_str(include_str!(config_path!("module/memory/editor.json"))).unwrap_ji());
}

pub fn get_init_word_ref(mode: GameMode, index: usize) -> Option<&'static str> {
    EDITOR_CONFIG 
        .get()
        .and_then(|config| {
            match mode {
                GameMode::Duplicate | GameMode::Lettering | GameMode::WordsAndImages => {
                    config.init.duplicate.get(index)
                },
                _ => unimplemented!("TODO")
            }
        })
        .map(|s| s.as_ref())
}

pub fn get_init_words(mode: GameMode) -> Vec<(String, String)> {
    EDITOR_CONFIG 
        .get()
        .map(|config| {
            match mode {
                GameMode::Duplicate | GameMode::Lettering => {
                    config.init.duplicate
                        .iter()
                        .map(|word| {
                            (word.to_string(), word.to_string())
                        })
                        .collect()
                },
                _ => unimplemented!("TODO")
            }
        })
        .unwrap_ji()
}
