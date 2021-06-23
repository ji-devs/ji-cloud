use crate::state::*;
use super::super::state::*;
use std::rc::Rc;

pub struct SidebarSettings {
    pub base: Rc<Base>
}

impl SidebarSettings {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base
        }
    }
}
