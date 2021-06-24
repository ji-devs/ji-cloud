use crate::state::Base;
use std::rc::Rc;
use shared::domain::jig::module::body::flashcards::ModuleData as RawData;
use futures_signals::signal::Mutable;

pub struct Settings {
    pub display_mode: Mutable<DisplayMode>
}

impl Settings {
    pub fn new(raw:&RawData) -> Self {
        Self {
            display_mode: Mutable::new(DisplayMode::Single)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayMode {
    Single,
    Pair
}

impl DisplayMode {
    pub fn as_str_id(&self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Pair => "pair",
        }
    }
}
