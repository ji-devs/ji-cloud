use super::super::state::*;
use crate::state::*;
use std::rc::Rc;

pub struct SidebarSettings {
    pub base: Rc<Base>,
}

impl SidebarSettings {
    pub fn new(base: Rc<Base>) -> Self {
        Self { base }
    }

    pub fn settings(&self) -> &Settings {
        &self.base.extra.settings
    }
}
