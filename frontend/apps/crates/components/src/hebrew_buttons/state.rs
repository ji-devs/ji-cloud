use std::rc::Rc;

use futures_signals::signal::Mutable;

pub struct HebrewButtons {
    pub(super) kind: Kind,
    pub(super) active_popup: Mutable<Option<Popup>>,
}

impl HebrewButtons {
    pub fn keyboard_only() -> Rc<Self> {
        Rc::new(Self {
            kind: Kind::KeyboardOnly,
            active_popup: Mutable::new(None),
        })
    }
    pub fn reveal() -> Rc<Self> {
        Rc::new(Self {
            kind: Kind::Reveal,
            active_popup: Mutable::new(None),
        })
    }

    pub fn full() -> Rc<Self> {
        Rc::new(Self {
            kind: Kind::Full,
            active_popup: Mutable::new(None),
        })
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Kind {
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
