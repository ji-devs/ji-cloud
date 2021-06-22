use shared::domain::jig::module::body::{ThemeId, _groups::cards::*};

pub enum Side {
    Left,
    Right
}


pub fn get_card_font_size(length: usize, theme_id: ThemeId) -> usize {
    //Todo - evaluate this...
    40
}
pub fn get_card_font_family(theme_id: ThemeId, mode: Mode, side: Side) -> usize {
    match mode {
        Mode::Lettering => {
            match theme_id {
                ThemeId::Blank => 1,
                ThemeId::Chalkboard => {
                    match side {
                        Side::Left => 2,
                        Side::Right => 3
                    }
                },
                ThemeId::HappyBrush => {
                    match side {
                        Side::Left => 1,
                        Side::Right => 2
                    }
                }
            }
        },
        _ => 1
    }
}
