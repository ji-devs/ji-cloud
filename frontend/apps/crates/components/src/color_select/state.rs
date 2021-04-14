use std::rc::Rc;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use rgb::RGBA8;
use utils::prelude::*;

use super::actions::{get_user_colors, hex_to_rgba8};


static SYSTEM_COLORS: &'static [&str] = &[
    "#00000000",
    "#ffffffff",
    "#fffcc7ff",
    "#fff445ff",
    "#feae2aff",
    "#f34826ff",
    "#ff0303ff",
    "#fdcdf1ff",
    "#f74ac8ff",
    "#da0f63ff",
    "#9517acff",
    "#7a28fbff",
    "#b0c7faff",
    "#2d9bf0ff",
    "#414cb3ff",
    "#09168dff",
    "#22bed9ff",
    "#1aa09dff",
    "#077472ff",
    "#8fd150ff",
    "#cfe741ff",
    "#cececeff",
    "#808080ff",
    "#1a1a1aff",
];


pub struct State {
    pub value: Rc<Mutable<Option<RGBA8>>>,
    pub system_colors: Rc<Vec<RGBA8>>,
    pub theme_colors: Rc<Option<Vec<RGBA8>>>,
    pub user_colors: Rc<MutableVec<RGBA8>>,
}

impl State {
    pub async fn new(config: ColorSelectConfig) -> Self {
        let user_colors = get_user_colors().await.unwrap_ji();
        Self {
            value: config.value.clone(),
            system_colors: Rc::new(SYSTEM_COLORS.iter().map(|c| hex_to_rgba8(*c)).collect()),
            theme_colors: Rc::new(match config.theme {
                // Some(ThemeId) => Some(THEME_COLORS.iter().map(|c| hex_to_rgba8(*c)).collect()),
                Some(theme_id) => Some(theme_id.get_colors().iter().map(|c| c.clone()).collect()),
                None => None,
            }),
            user_colors: Rc::new(MutableVec::new_with_values(user_colors)),
        }
    }
}

pub struct ColorSelectConfig {
    pub theme: Option<ThemeId>,
    pub value: Rc<Mutable<Option<RGBA8>>>,
}
