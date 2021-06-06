use components::module::edit::prelude::*;
use super::super::state::Base;
use std::rc::Rc;

pub struct Header {
    pub base: Rc<Base>
}


impl Header {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base 
        }
    }
}

impl HeaderExt for Header{
}
