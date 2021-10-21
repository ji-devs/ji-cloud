use std::rc::Rc;

use futures_signals::signal::Mutable;

pub struct HebrewButtons {
    pub(super) full: bool,
    pub(super) active_popup: Mutable<Option<Popup>>,
}

impl HebrewButtons {
    pub fn short() -> Rc<Self> {
        Rc::new(Self {
            full: false,
            active_popup: Mutable::new(None),
        })
    }

    pub fn full() -> Rc<Self> {
        Rc::new(Self {
            full: true,
            active_popup: Mutable::new(None),
        })
    }
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
