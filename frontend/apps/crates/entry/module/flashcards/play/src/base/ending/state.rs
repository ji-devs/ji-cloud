use crate::base::state::Base;
use std::rc::Rc;

pub struct Ending {
    pub base: Rc<Base>,
}

impl Ending {
    pub fn new(base: Rc<Base>) -> Self {
        Self { base }
    }
}
