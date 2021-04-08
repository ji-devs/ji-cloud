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
    single_list_words: Vec<String>,
    dual_list_words: Vec<(String, String)>,
}

pub fn init() {
    EDITOR_CONFIG.set(serde_json::from_str(include_str!(config_path!("module/memory/editor.json"))).unwrap_ji());
}

pub fn get_single_list_init_word(index: usize) -> Option<&'static str> {
    EDITOR_CONFIG 
        .get()
        .and_then(|config| {
            config.init.single_list_words.get(index)
        })
        .map(|s| s.as_ref())
}

pub fn get_dual_list_init_word(row: usize, col: usize) -> Option<&'static str> {
    EDITOR_CONFIG 
        .get()
        .and_then(|config| {
            config.init.dual_list_words.get(row)
                .map(|words| {
                    if col == 0 {
                       &words.0 
                    } else if col == 1 {
                       &words.1
                    } else {
                        panic!("no such column!");
                    }
                })
        })
        .map(|s| s.as_ref())
}
pub fn get_debug_pairs(mode: GameMode) -> Vec<(String, String)> {
    EDITOR_CONFIG 
        .get()
        .map(|config| {
            match mode {
                GameMode::Duplicate | GameMode::Lettering => {
                    config.init.single_list_words
                        .iter()
                        .map(|word| {
                            (word.to_string(), word.to_string())
                        })
                        .collect()
                },
                GameMode::WordsAndImages => {
                    config.init.single_list_words
                        .iter()
                        .map(|word| {
                            (word.to_string(), "".to_string())
                        })
                        .collect()
                },
                _ => {
                    config.init.dual_list_words.clone()
                }
            }
        })
        .unwrap_ji()
}
