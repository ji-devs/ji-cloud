use dominator::{Dom, DomBuilder, clone, html};
use utils::prelude::*;
use web_sys::HtmlElement;
use crate::module::_groups::cards::lookup::{self, Side};
use shared::domain::jig::module::body::{ModeExt, Transform, _groups::cards::{Mode, Step, Card}};
use futures_signals::signal::{Signal, SignalExt, Always};

//must match @elements/module/_groups/cards/play/card/styles.ts
//export type Size = "memory" | "flashcards" | "quiz-option" | "quiz-target" | "matching";
pub enum Size {
    Memory,
    Flashcards,
    QuizOption,
    QuizTarget,
    Matching
}

impl Size {
    pub fn as_str_id(&self) -> &'static str {
        match self {
            Self::Memory => "memory",
            Self::Flashcards => "flashcards",
            Self::QuizOption => "quiz-option",
            Self::QuizTarget => "quiz-target",
            Self::Matching => "matching"
        }
    }
}

pub struct SimpleTransform {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
}


pub(super) fn render_media(card:&Card, mode: Mode, theme_id: ThemeId, slot: Option<&str>) -> Dom {
    match &card {
        Card::Text(s) => {
            html!("card-text", {
                .apply_if(slot.is_some(), |dom| {
                    dom.property("slot", slot.unwrap_ji())
                })
                .property("value", s)
                .property("fontSize", {
                    let font_size = lookup::get_card_font_size(s.len(), theme_id, mode);
                    format!("{}rem", font_size)
                })
            })
        },
        Card::Image(image) => {
            match image {
                Some(image) => {
                    image.render(slot)
                },
                None => {
                    html!("img-ui", {
                        .apply_if(slot.is_some(), |dom| {
                            dom.property("slot", slot.unwrap_ji())
                        })
                        .property("path", "core/_common/image-empty.svg")
                    })
                }
            }
        },
    }
}
