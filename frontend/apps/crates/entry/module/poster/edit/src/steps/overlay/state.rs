use components::module::edit::prelude::*;
use super::super::state::Base;
use std::rc::Rc;

pub struct Overlay {
    pub base: Rc<Base>
}


impl Overlay {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base 
        }
    }
}

impl OverlayExt for Overlay {
}
