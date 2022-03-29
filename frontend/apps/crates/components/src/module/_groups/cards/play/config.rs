/** For now, dual-purposing editor and player config */
use once_cell::sync::OnceCell;

use serde::Deserialize;
use shared::domain::jig::module::body::_groups::cards::Mode;
use utils::prelude::*;

pub const MAX_LIST_WORDS: usize = 14;

pub static DUAL_LIST_CHAR_LIMIT: usize = 30;
pub static SINGLE_LIST_CHAR_LIMIT: usize = 30;
pub static CARD_TEXT_LIMIT_WIDTH: f64 = 150.0;
pub static CARD_TEXT_LIMIT_HEIGHT: f64 = 150.0;

macro_rules! config_path {
    ($e:tt) => {
        concat!("../../../../../../../../config/", $e)
    };
}
static EDITOR_CONFIG: OnceCell<EditorConfig> = OnceCell::new();

#[derive(Debug, Deserialize)]
struct EditorConfig {
    init: InitConfig,
}
#[derive(Debug, Deserialize)]
struct InitConfig {
    single_list_words: Vec<String>,
    dual_list_words: Vec<(String, String)>,
}

pub fn init() {
    let _ = EDITOR_CONFIG.set(
        serde_json::from_str(include_str!(config_path!(
            "module/_groups/cards/editor.json"
        )))
        .unwrap_ji(),
    );
}

pub fn get_single_list_init_word(index: usize) -> Option<String> {
    EDITOR_CONFIG
        .get()
        .and_then(|config| config.init.single_list_words.get(index))
        .map(|s| s.to_string())
}

pub fn get_dual_list_init_word(row: usize, col: usize) -> Option<String> {
    EDITOR_CONFIG
        .get()
        .and_then(|config| {
            config.init.dual_list_words.get(row).map(|words| {
                if col == 0 {
                    &words.0
                } else if col == 1 {
                    &words.1
                } else {
                    panic!("no such column!");
                }
            })
        })
        .map(|s| s.to_string())
}

pub fn get_debug_pairs(mode: Mode) -> Vec<(String, String)> {
    EDITOR_CONFIG
        .get()
        .map(|config| match mode {
            Mode::Duplicate => config
                .init
                .single_list_words
                .iter()
                .skip(1)
                .map(|word| (word.to_string(), word.to_string()))
                .collect(),
            Mode::WordsAndImages => config
                .init
                .single_list_words
                .iter()
                .map(|word| (word.to_string(), "".to_string()))
                .collect(),
            // Images/Images doesn't use lists at all
            Mode::Images => vec![("".to_string(), "".to_string())],
            _ => config.init.dual_list_words.clone(),
        })
        .unwrap_ji()
}
