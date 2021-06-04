
use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use serde::Deserialize;
use utils::prelude::*;
use shared::domain::jig::module::body::memory::Mode;

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


pub fn get_debug_pairs(mode: Mode, n_cards:usize) -> Vec<(String, String)> {
    match mode {
        Mode::Duplicate | Mode::Lettering => {
            let mut cards = Vec::new();

            for i in 0..n_cards {
                cards.push(("hello".to_string(), "world".to_string()));
            }
            cards
        },
        Mode::WordsAndImages => {
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
