use super::super::state::Base;
use components::module::_common::edit::prelude::*;
use std::rc::Rc;
pub struct Overlay {
    pub base: Rc<Base>,
}

impl Overlay {
    pub fn new(base: Rc<Base>) -> Self {
        Self { base }
    }
}

impl OverlayExt for Overlay {}
