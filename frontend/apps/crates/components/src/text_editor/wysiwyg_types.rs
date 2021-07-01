// this file needs to be in sync with frontend\elements\src\core\wysiwyg\wysiwyg-types.ts

use serde::{ Serialize, Deserialize};
use wasm_bindgen::JsValue;

use strum_macros::{EnumIter, Display};
use dominator_helpers::make_custom_event_serde;

use super::{super::font_loader::Font as StaticFont, font_css_converter::font_to_css};

#[derive(Clone, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
pub enum ElementType {
    H1,
    H2,
    P1,
    P2,
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
pub type Weight = u16;
pub type Font = String;
pub type Color = String;

pub const BOLD_WEIGHT: Weight = 700;
pub const REGULAR_WEIGHT: Weight = 400;

#[derive(Clone, Serialize, Deserialize)]
pub struct ControlsState {
    pub font: Font,
    pub element: ElementType,
    pub weight: Weight,
    pub align: Align,
    pub font_size: FontSize,
    // using strings for color as it's easy to and from convert to js
    pub color: Option<Color>,
    pub highlight_color: Option<Color>,
    pub indent_count: IndentCount,
    pub italic: bool,
    pub underline: bool,
}

impl ControlsState {
    // maybe take from js default
    pub fn new() -> Self {
        Self {
            font: String::from(StaticFont::RobotoSlabRegular.get_font_name()),
            element: ElementType::P1,
            weight: REGULAR_WEIGHT,
            align: Align::Left,
            font_size: 16,
            color: None,
            highlight_color: None,
            indent_count: 0,
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
    Italic(bool),
    Underline(bool),
}

impl ControlsChange {
    pub fn to_js_key_value(&self) -> (JsValue, JsValue) {
        let key = enum_variant_to_string(self);
        let key = JsValue::from_str(&key);

        let value = match self {
            Self::Font(font) => JsValue::from_str(&font_to_css(&font.to_string())),
            Self::Element(element) => JsValue::from_str(&element.to_string()),
            Self::Weight(weight) => JsValue::from_f64(*weight as f64),
            Self::Align(align) => JsValue::from_str(&align.to_string()),
            Self::FontSize(font_size) => JsValue::from_f64(*font_size as f64),
            Self::IndentCount(indent_count) => JsValue::from_f64(*indent_count as f64),
            Self::Italic(italic) => JsValue::from_bool(*italic),
            Self::Underline(underline) => JsValue::from_bool(*underline),
            Self::Color(color) => {
                match color {
                    Some(color) => JsValue::from_str(&color),
                    None => JsValue::UNDEFINED,
                }
            },
            Self::HighlightColor(highlight_color) => {
                match highlight_color {
                    Some(highlight_color) => JsValue::from_str(&highlight_color),
                    None => JsValue::UNDEFINED,
                }
            },
        };

        (key, value)
    }
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
