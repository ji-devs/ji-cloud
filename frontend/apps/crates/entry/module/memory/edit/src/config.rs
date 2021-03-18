use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use serde::Deserialize;
use crate::data::state::Theme;

macro_rules! config_path {
    ($e:tt) => { 
        concat!("../../../../../../../config/", $e)
    } 
}
pub static INITIAL_WORDS:OnceCell<Vec<String>> = OnceCell::new();
pub static THEME_CHOICES:OnceCell<Vec<Theme>> = OnceCell::new();

pub const THEME_EXAMPLE_TEXT_1:&'static str = "שמש";
pub const THEME_EXAMPLE_TEXT_2:&'static str = "sun";

#[derive(Deserialize)]
struct InitWords {
    words: Vec<String>
}

#[derive(Deserialize)]
struct ThemeChoices {
    themes: Vec<Theme>
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
