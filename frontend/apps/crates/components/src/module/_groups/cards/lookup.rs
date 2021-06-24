use shared::domain::jig::module::body::{ThemeId, _groups::cards::*};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Side {
    Left,
    Right
}

impl Side {
    pub const fn as_str_id(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

pub fn get_card_font_size(length: usize, theme_id: ThemeId, mode: Mode) -> usize {
    //Todo - evaluate this...
    40
}