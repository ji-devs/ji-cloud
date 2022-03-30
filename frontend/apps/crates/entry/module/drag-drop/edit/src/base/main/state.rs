use crate::base::{sidebar::state::Sidebar, state::Base};
use components::module::_common::edit::prelude::*;
use std::rc::Rc;

pub struct Main {
    pub base: Rc<Base>,
    pub sidebar: Rc<Sidebar>,
}

impl Main {
    pub fn new(base: Rc<Base>, sidebar: Rc<Sidebar>) -> Self {
        Self { base, sidebar }
    }
}

impl MainExt for Main {}
