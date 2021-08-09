use futures_signals::signal::{Mutable, ReadOnlyMutable};

use super::callbacks::Callbacks;
use shared::domain::jig::module::body::{
    Background, ThemeId, _groups::design::Backgrounds as RawBackgrounds,
};

pub struct Backgrounds {
    pub theme_id: ReadOnlyMutable<ThemeId>,
    pub layer_1: Mutable<Option<Background>>,
    pub layer_2: Mutable<Option<Background>>,
    pub(super) callbacks: Callbacks,
}
impl Backgrounds {
    pub fn from_raw(
        raw: &RawBackgrounds,
        theme_id: ReadOnlyMutable<ThemeId>,
        callbacks: Callbacks,
    ) -> Self {
        Self {
            theme_id,
            layer_1: Mutable::new(raw.layer_1.clone()),
            layer_2: Mutable::new(raw.layer_2.clone()),
            callbacks,
        }
    }

    pub fn to_raw(&self) -> RawBackgrounds {
        RawBackgrounds {
            layer_1: self.layer_1.get_cloned(),
            layer_2: self.layer_2.get_cloned(),
        }
    }
}
