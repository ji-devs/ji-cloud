use std::rc::Rc;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use rgb::RGBA8;
use utils::{prelude::*, colors::*};



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
    pub(super) value: Mutable<Option<RGBA8>>,
    pub system_colors: Rc<Vec<RGBA8>>,
    pub theme_colors: Mutable<Option<Vec<RGBA8>>>,
    pub user_colors: Rc<MutableVec<RGBA8>>,
    pub on_select: Option<Box<dyn Fn(RGBA8)>>,
}

impl State {
    pub fn new(theme: Option<ThemeId>, init_value: Option<RGBA8>, on_select: Option<impl Fn(RGBA8) + 'static>) -> Self {
        Self {
            value: Mutable::new(init_value), 
            system_colors: Rc::new(SYSTEM_COLORS.iter().map(|c| hex_to_rgba8(*c)).collect()),
            theme_colors: Mutable::new(match theme {
                Some(theme_id) => Some(Self::get_theme_colors(theme_id)),
                None => None,
            }),
            user_colors: Rc::new(MutableVec::new()),
            on_select: on_select.map(|f| Box::new(f) as _)
        }
    }

    pub fn set_theme(&self, theme_id: ThemeId) {
        self.theme_colors.set(Some(Self::get_theme_colors(theme_id)))
    }

    fn get_theme_colors(theme_id: ThemeId) -> Vec<RGBA8> {
        theme_id.get_colors().iter().map(|c| c.clone()).collect()
    }
}
