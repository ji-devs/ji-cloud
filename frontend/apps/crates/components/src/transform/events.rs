use serde::Deserialize;
use dominator_helpers::{temp_make_event, make_custom_event_serde, make_custom_event};
use web_sys::{File, DomRect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use super::state::ScaleFrom;

#[derive(Deserialize, Debug)]
pub struct RectDblClickData {
    pub x: f64,
    pub y: f64,
}

make_custom_event_serde!("transform-rect-dblclick", RectDblClick, RectDblClickData);

#[derive(Deserialize, Debug)]
pub struct MoveData {
    pub x: f64,
    pub y: f64,
}

make_custom_event_serde!("transform-move-start", Move, MoveData);

#[derive(Deserialize, Debug)]
pub struct RotateData {
    pub x: f64,
    pub y: f64,
}

make_custom_event_serde!("transform-rotate-start", Rotate, RotateData);


#[derive(Deserialize, Debug)]
pub struct ResizeData {
    pub pos: String,
    pub x: f64,
    pub y: f64,
}

make_custom_event_serde!("transform-resize-start", Resize, ResizeData);

impl ResizeData {
    pub fn scale_from(&self) -> ScaleFrom {
        match self.pos.as_ref() {
            "tl" => ScaleFrom::TopLeft,
            "t" => ScaleFrom::Top,
            "tr" => ScaleFrom::TopRight,
            "l" => ScaleFrom::Left,
            "bl" => ScaleFrom::BottomLeft,
            "b" => ScaleFrom::Bottom,
            "br" => ScaleFrom::BottomRight,
            "r" => ScaleFrom::Right,
            _ => panic!("unknown scale from!")
        }
    }
}
