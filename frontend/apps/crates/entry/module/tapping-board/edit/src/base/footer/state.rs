use components::module::edit::*;
use super::super::state::Base;
use std::rc::Rc;

pub struct Footer {
    pub base: Rc<Base>
}


impl Footer {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base 
        }
    }
}

impl FooterExt for Footer {
}
