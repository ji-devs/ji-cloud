use utils::{prelude::*, colors::*};

use super::wysiwyg_types::{ElementType, Font, FontSize, Color};

pub fn get_theme_element_styles(theme_id: &ThemeId, element: &ElementType) -> (Font, Color, FontSize) {
    let lookup: (usize, usize, FontSize) = match theme_id {
        ThemeId::None => {
            match element {
                ElementType::H1 => (0, 1, 28),
                ElementType::H2 => (1, 2, 25),
                ElementType::P1 => (0, 1, 14),
                ElementType::P2 => (1, 2, 12),
            }
        }
        ThemeId::Chalkboard => {
            match element {
                ElementType::H1 => (0, 1, 28),
                ElementType::H2 => (1, 2, 25),
                ElementType::P1 => (0, 1, 14),
                ElementType::P2 => (1, 2, 12),
            }
        }
        ThemeId::HappyBrush => {
            match element {
                ElementType::H1 => (0, 1, 28),
                ElementType::H2 => (1, 2, 25),
                ElementType::P1 => (0, 1, 14),
                ElementType::P2 => (1, 2, 12),
            }
        }
    };

    (
        theme_id.get_fonts()[lookup.0].clone(),
        rgba8_to_hex(&theme_id.get_colors()[lookup.1]),
        lookup.2
    )
}
