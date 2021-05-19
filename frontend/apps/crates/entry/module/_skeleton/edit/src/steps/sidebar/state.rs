use components::module::edit::*;
use super::super::state::Base;
use std::rc::Rc;

pub struct Sidebar {
    pub base: Rc<Base>
}


impl Sidebar {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base 
        }
    }
}

impl SidebarExt for Sidebar {
}
