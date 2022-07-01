use super::super::state::Base;
use components::module::_common::edit::prelude::*;
use std::rc::Rc;

pub struct Header {
    pub base: Rc<Base>,
}

impl Header {
    pub fn new(base: Rc<Base>) -> Self {
        Self { base }
    }
}

impl HeaderExt for Header {}
