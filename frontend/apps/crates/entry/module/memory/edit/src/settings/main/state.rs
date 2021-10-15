use crate::state::*;

use std::rc::Rc;

pub struct MainSettings {
    pub base: Rc<Base>,
}

impl MainSettings {
    pub fn new(base: Rc<Base>) -> Self {
        Self { base }
    }
}
