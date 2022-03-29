use super::play::card::dom::Size;
use unicode_segmentation::UnicodeSegmentation;

const FONT_SIZE_RANGE: (f32, f32) = (200f32, 60f32);
const TEXT_LENGTH_RANGE: (usize, usize) = (1, 10);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub const fn as_str_id(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    pub fn negate(&self) -> Self {
        if *self == Side::Left {
            Side::Right
        } else {
            Side::Left
        }
    }
}

pub fn get_card_font_size(value: &str, size: Option<&Size>) -> String {
    let size = match size {
        None => 40f32, // Return the original font size
        Some(size) => {
            let size_scale = match size {
                Size::Flashcards => 1f32,
                Size::QuizTarget => 0.80f32,
                Size::Matching | Size::QuizOption => 0.45f32,
                Size::Memory => 0.3f32,
            };

            let value_len = value.graphemes(true).count();

            // Different card games have different sized cards, this scales the final font size per
            // card size.
            let font_size_range = (
                FONT_SIZE_RANGE.0 * size_scale,
                FONT_SIZE_RANGE.1 * size_scale,
            );
            let scale = (font_size_range.1 - font_size_range.0 as f32)
                / (TEXT_LENGTH_RANGE.1 as f32 - TEXT_LENGTH_RANGE.0 as f32);
            let capped = std::cmp::min(
                TEXT_LENGTH_RANGE.1,
                std::cmp::max(TEXT_LENGTH_RANGE.0, value_len),
            ) - TEXT_LENGTH_RANGE.0;

            capped as f32 * scale + font_size_range.0 as f32
        }
    };

    format!("{:.2}rem", size)
}
