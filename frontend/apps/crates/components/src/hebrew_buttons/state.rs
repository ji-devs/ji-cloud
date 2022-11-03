use std::rc::Rc;

use dominator_helpers::make_custom_event;
use futures_signals::signal::Mutable;
use wasm_bindgen::JsValue;

use super::HebrewButtonsConfig;

pub struct HebrewButtons {
    pub(super) kind: Kind,
    pub(super) active_popup: Mutable<Option<Popup>>,
    pub(super) on_open_toggle: Box<dyn Fn(bool)>,
}

impl HebrewButtons {
    pub fn new(config: HebrewButtonsConfig) -> Rc<Self> {
        let active_popup = Mutable::new(None);
        Rc::new(Self {
            active_popup,
            kind: config.kind,
            on_open_toggle: config.on_open_toggle,
        })
    }
    pub fn keyboard_only() -> Rc<Self> {
        Self::new(HebrewButtonsConfig {
            kind: Kind::KeyboardOnly,
            ..Default::default()
        })
    }
    pub fn reveal() -> Rc<Self> {
        Self::new(HebrewButtonsConfig {
            kind: Kind::Reveal,
            ..Default::default()
        })
    }
    pub fn full() -> Rc<Self> {
        Self::new(HebrewButtonsConfig {
            kind: Kind::Full,
            ..Default::default()
        })
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    KeyboardOnly,
    Reveal,
    Full,
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Popup {
    Keyboard,
    Dicta,
    Sefaria,
}

impl Popup {
    pub fn str(&self) -> &str {
        match self {
            Self::Keyboard => "keyboard",
            Self::Dicta => "dicta",
            Self::Sefaria => "sefaria",
        }
    }
}

// used to close other hebrew-buttons-popups
make_custom_event!(HebrewButtonOpened, "hebrew-button-opened");
