use serde::Deserialize;
use dominator_helpers::{temp_make_event, make_custom_event_serde, make_custom_event};
use web_sys::{File, DomRect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Deserialize, Debug)]
pub struct MoveData {
    pub x: f64,
    pub y: f64,
}

make_custom_event_serde!("transform-move-start", Move, MoveData);

impl Move {
    pub fn pos(&self) -> (f64, f64) {
        let MoveData { x, y} = self.data();
        (x, y)
    }
}
