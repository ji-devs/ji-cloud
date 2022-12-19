use crate::base::state::Base;
use std::rc::Rc;

pub struct PlaySettingsState {
    pub base: Rc<Base>,
}

impl PlaySettingsState {
    pub fn new(base: Rc<Base>) -> Self {
        Self { base }
    }
}
