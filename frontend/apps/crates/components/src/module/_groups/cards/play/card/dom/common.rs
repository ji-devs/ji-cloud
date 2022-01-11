use dominator::{html, Dom};
use utils::prelude::*;
use wasm_bindgen::JsValue;

use crate::module::_groups::cards::lookup;
use shared::domain::jig::module::body::_groups::cards::Card;

//must match @elements/module/_groups/cards/play/card/styles.ts
//export type Size = "memory" | "flashcards" | "quiz-option" | "quiz-target" | "matching";
pub enum Size {
    Memory,
    Flashcards,
    QuizOption,
    QuizTarget,
    Matching,
}

impl Size {
    pub fn as_str_id(&self) -> &'static str {
        match self {
            Self::Memory => "memory",
            Self::Flashcards => "flashcards",
            Self::QuizOption => "quiz-option",
            Self::QuizTarget => "quiz-target",
            Self::Matching => "matching",
        }
    }
}

pub enum Effect {
    Positive,
    Negative,
    None,
}

impl From<Effect> for JsValue {
    fn from(effect: Effect) -> JsValue {
        match effect {
            Effect::Positive => JsValue::from_str("positive"),
            Effect::Negative => JsValue::from_str("negative"),
            Effect::None => JsValue::NULL,
        }
    }
}

pub enum StyleKind {
    Theme,
    None,
    Dragging,
}

impl StyleKind {
    pub fn as_str_id(&self) -> &'static str {
        match self {
            Self::Theme => "theme",
            Self::None => "none",
            Self::Dragging => "dragging",
        }
    }
}
pub struct SimpleTransform {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
}

pub(super) fn render_media(card: &Card, size: &Size, slot: Option<&str>) -> Dom {
    match &card {
        Card::Text(s) => {
            html!("card-text", {
                .apply_if(slot.is_some(), |dom| {
                    dom.property("slot", slot.unwrap_ji())
                })
                .property("value", s)
                .property("fontSize", {
                    lookup::get_card_font_size(s, Some(size))
                })
            })
        }
        Card::Image(image) => match image {
            Some(image) => image.render(slot),
            None => {
                html!("img-ui", {
                    .apply_if(slot.is_some(), |dom| {
                        dom.property("slot", slot.unwrap_ji())
                    })
                    .property("path", "core/_common/image-empty.svg")
                })
            }
        },
    }
}
