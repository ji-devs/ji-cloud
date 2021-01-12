use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use serde::Deserialize;

pub const TRANISITION_DURATION:f64 = 3000.0;
pub const DEST_X_LEFT:f64 = 5.0;
pub const DEST_Y_START:f64 = 10.0;
pub const DEST_LINE_OFFSET:f64 = 20.0;
pub const DEST_ROT_LEFT:f64 = 0.0;
pub const DEST_X_RIGHT:f64 = 15.0;
pub const DEST_ROT_RIGHT:f64 = -20.0;

macro_rules! config_path {
    ($e:tt) => { 
        concat!("../../../../../../../config/", $e)
    } 
}
pub static INITIAL_WORDS:OnceCell<Vec<String>> = OnceCell::new();
pub static THEME_CHOICES:OnceCell<Vec<Theme>> = OnceCell::new();

#[derive(Deserialize)]
struct InitWords {
    words: Vec<String>
}

#[derive(Deserialize)]
struct ThemeChoices {
    themes: Vec<Theme>
}
#[derive(Deserialize, Clone)]
pub struct Theme {
    pub id: String,
    pub content: String,
    pub label: String
}

pub fn init() {
    let json:InitWords = serde_json::from_str(include_str!(config_path!("module/memory/initial-words.json"))).unwrap_throw();
    INITIAL_WORDS.set(json.words);
    let json:ThemeChoices = serde_json::from_str(include_str!(config_path!("module/memory/themes.json"))).unwrap_throw();
    THEME_CHOICES.set(json.themes);
}

pub fn get_init_words_cloned() -> Vec<String> {
    INITIAL_WORDS
        .get()
        .map(|x| x.clone())
        .unwrap_throw()
}


pub fn get_init_words_iter() -> impl Iterator<Item = &'static String> {
    INITIAL_WORDS
        .get()
        .unwrap_throw()
        .iter()
}

pub fn get_init_words_string() -> String { 
    INITIAL_WORDS
        .get()
        .unwrap_throw()
        .iter()
        .fold(String::new(), |acc, curr| {
            if acc.is_empty() {
                curr.to_string()
            } else {
                format!("{}\n{}", acc, curr)
            }
        })
}

pub fn get_themes_cloned() -> Vec<Theme> { 
    THEME_CHOICES 
        .get()
        .map(|x| x.clone())
        .unwrap_throw()
}
pub fn get_themes_iter() -> impl Iterator<Item = &'static Theme> {
    THEME_CHOICES 
        .get()
        .unwrap_throw()
        .iter()
}
