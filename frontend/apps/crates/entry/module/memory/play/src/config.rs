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

pub const TRANISITION_DURATION:f64 = 3000.0;
pub const DEST_X_LEFT:f64 = 5.0;
pub const DEST_Y_START:f64 = 10.0;
pub const DEST_LINE_OFFSET:f64 = 20.0;
pub const DEST_ROT_LEFT:f64 = 0.0;
pub const DEST_X_RIGHT:f64 = 15.0;
pub const DEST_ROT_RIGHT:f64 = -20.0;


pub fn get_debug_pairs(mode: GameMode) -> Vec<(String, String)> {
    match mode {
        GameMode::Duplicate | GameMode::Lettering => {
            vec![("hello", "world")]
                .iter()
                .map(|(w1, w2)| (w1.to_string(), w2.to_string()))
                .collect()
        },
        GameMode::WordsAndImages => {
            vec![("hello", "")]
                .iter()
                .map(|(w1, w2)| (w1.to_string(), w2.to_string()))
                .collect()
        },
        _ => {
            vec![("hello", "world")]
                .iter()
                .map(|(w1, w2)| (w1.to_string(), w2.to_string()))
                .collect()
        }
    }
}
