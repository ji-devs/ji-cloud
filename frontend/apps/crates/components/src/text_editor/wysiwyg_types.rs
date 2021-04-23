// this file needs to be in sync with frontend\elements\src\core\wysiwyg\wysiwyg-types.ts

use std::rc::Rc;

use serde::{ Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

use futures_signals::signal::Mutable;
use rgb::RGBA8;
use strum_macros::{EnumIter, Display};
use dominator_helpers::make_custom_event_serde;



#[derive(Clone, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Font {
    Arial,
    Roboto,
    OpenSans,
}

#[derive(Clone, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
pub enum ElementType {
    H1,
    H2,
    P1,
    P2,
}

#[derive(Clone, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Weight {
    Bolder,
    Bold,
    Normal,
    Lighter,
}

#[derive(Clone, Debug, Display, EnumIter, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Align {
    Left,
    Center,
    Right,
}

pub type FontSize = u8;
pub type IndentCount = u8;

#[derive(Clone, Serialize, Deserialize)]
pub struct ControlsState {
    pub font: Font,
    pub element: ElementType,
    pub weight: Weight,
    pub align: Align,
    pub font_size: FontSize,
    // using strings for color as it's easy to and from convert to js
    pub color: Option<String>,
    pub highlight_color: Option<String>,
    pub indent_count: IndentCount,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl ControlsState {
    // maybe take from js default
    pub fn new() -> Self {
        Self {
            font: Font::Arial,
            element: ElementType::P1,
            weight: Weight::Normal,
            align: Align::Left,
            font_size: 10,
            color: None,
            highlight_color: None,
            indent_count: 0,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ControlsChange {
    Font(Font),
    Element(ElementType),
    Weight(Weight),
    Align(Align),
    FontSize(FontSize),
    Color(Option<String>),
    HighlightColor(Option<String>),
    IndentCount(IndentCount),
    Bold(bool),
    Italic(bool),
    Underline(bool),
}

make_custom_event_serde!("wysiwyg-controls-change", WysiwygControlsChange, ControlsChange);

impl WysiwygControlsChange {
    pub fn value(&self) -> ControlsChange {
        self.detail().into_serde().unwrap()
    }
}


pub fn enum_variant_to_string<T: ?Sized>(v: &T) -> String 
where
    T: Serialize
{
    let s = serde_json::to_string(&v).unwrap();
    let chars = s.chars();
    let start = 2;
    let mut end = 3;
    for (i, c) in chars.enumerate().skip(2) {
        if c == '"' {
            end = i;
            break;
        }
    }
    let s = String::from(&s[start..end]);
    s
}
