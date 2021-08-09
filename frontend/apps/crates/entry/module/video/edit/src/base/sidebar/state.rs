use crate::base::state::Base;
use components::module::_common::edit::prelude::*;
use std::rc::Rc;

pub struct Sidebar {
    pub base: Rc<Base>,
}

impl Sidebar {
    pub fn new(base: Rc<Base>) -> Self {
        Self { base }
    }
}

impl SidebarExt for Sidebar {}
