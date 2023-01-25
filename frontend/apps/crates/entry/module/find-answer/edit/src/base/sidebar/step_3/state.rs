use std::rc::Rc;

use components::tabs::MenuTabKind;
use futures_signals::signal::Mutable;

use super::super::state::Sidebar;

pub struct Step3 {
    pub sidebar: Rc<Sidebar>,
    pub advanced_visible: Mutable<bool>,
}

impl Step3 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        Rc::new(Self {
            sidebar,
            advanced_visible: Mutable::new(false),
        })
    }
}

#[derive(Clone)]
pub enum Tab {
    Question,
    Answer,
}

impl From<MenuTabKind> for Tab {
    fn from(kind: MenuTabKind) -> Self {
        match kind {
            MenuTabKind::Answer => Self::Answer,
            MenuTabKind::Question => Self::Question,
            _ => unimplemented!("Unsupported tab kind"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdvancedFeedbackSelection {
    All,
    Selected,
}
