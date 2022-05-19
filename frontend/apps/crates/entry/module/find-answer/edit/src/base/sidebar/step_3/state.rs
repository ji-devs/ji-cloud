use std::rc::Rc;

use components::tabs::MenuTabKind;

use super::super::state::Sidebar;

pub struct Step3 {
    pub sidebar: Rc<Sidebar>,
}

impl Step3 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        Rc::new(Self { sidebar })
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
