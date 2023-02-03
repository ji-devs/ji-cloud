// this file needs to be in sync with frontend\elements\src\core\wysiwyg\wysiwyg-types.ts

use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use utils::unwrap::UnwrapJiExt;
use wasm_bindgen::JsValue;

use dominator_helpers::make_custom_event_serde;
use strum_macros::{Display, EnumIter};

use super::font_css_converter::font_to_css;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Display)]
#[serde(rename_all = "PascalCase")]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

impl Default for Direction {
    fn default() -> Self {
        Self::LeftToRight
    }
}

#[derive(Clone, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
pub enum ElementType {
    H1,
    H2,
    P1,
    P2,
}

impl Default for ElementType {
    fn default() -> Self {
        Self::H1
    }
}

#[derive(Clone, Debug, Display, EnumIter, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Default for Align {
    fn default() -> Self {
        Self::Left
    }
}

pub type FontSize = u8;
pub type Weight = u16;
pub type Font = String;
pub type Color = String;

pub const BOLD_WEIGHT: Weight = 700;
pub const REGULAR_WEIGHT: Weight = 400;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlsState {
    pub font: Font,
    pub element: ElementType,
    pub weight: Weight,
    pub align: Align,
    pub font_size: FontSize,
    // using strings for color as it's easy to and from convert to js
    pub color: Option<Color>,
    pub highlight_color: Option<Color>,
    pub box_color: Option<Color>,
    pub italic: bool,
    pub underline: bool,
}

impl Default for ControlsState {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlsState {
    // maybe take from js default
    pub fn new() -> Self {
        Self {
            font: super::config::DEFAULT_FONT_FAMILY.to_string(),
            element: ElementType::default(),
            weight: REGULAR_WEIGHT,
            align: Align::default(),
            font_size: 16,
            color: None,
            highlight_color: None,
            box_color: None,
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
    BoxColor(Option<String>),
    Direction(Direction),
    Italic(bool),
    Underline(bool),
}

impl ControlsChange {
    pub fn to_js_key_value(&self) -> (JsValue, JsValue) {
        let key = enum_variant_to_string(self);
        let key = JsValue::from_str(&key);

        let value = match self {
            Self::Font(font) => JsValue::from_str(&font_to_css(font)),
            Self::Element(element) => JsValue::from_str(&element.to_string()),
            Self::Weight(weight) => JsValue::from_f64(*weight as f64),
            Self::Align(align) => JsValue::from_str(&align.to_string()),
            Self::FontSize(font_size) => JsValue::from_f64(*font_size as f64),
            Self::Direction(direction) => JsValue::from_str(&direction.to_string()),
            Self::Italic(italic) => JsValue::from_bool(*italic),
            Self::Underline(underline) => JsValue::from_bool(*underline),
            Self::Color(color) => match color {
                Some(color) => JsValue::from_str(color),
                None => JsValue::UNDEFINED,
            },
            Self::HighlightColor(highlight_color) => match highlight_color {
                Some(highlight_color) => JsValue::from_str(highlight_color),
                None => JsValue::UNDEFINED,
            },
            Self::BoxColor(highlight_color) => match highlight_color {
                Some(box_color) => JsValue::from_str(box_color),
                None => JsValue::UNDEFINED,
            },
        };

        (key, value)
    }
}

make_custom_event_serde!(
    "wysiwyg-controls-change",
    WysiwygControlsChange,
    ControlsState
);

impl WysiwygControlsChange {
    pub fn value(&self) -> ControlsState {
        // log::info!("{:#?}", self.detail());

        JsValueSerdeExt::into_serde(&self.detail()).unwrap_ji()
    }
}

pub fn enum_variant_to_string<T: ?Sized>(v: &T) -> String
where
    T: Serialize,
{
    let s = serde_json::to_string(&v).unwrap_ji();
    let chars = s.chars();
    let start = 2;
    let mut end = 3;
    for (i, c) in chars.enumerate().skip(2) {
        if c == '"' {
            end = i;
            break;
        }
    }

    String::from(&s[start..end])
}
