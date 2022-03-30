use crate::base::state::Base;
use std::rc::Rc;
// pub const DEFAULT_SELECT_AMOUNT: usize = 3;

pub struct PlaySettingsState {
    pub base: Rc<Base>,
}

impl PlaySettingsState {
    pub fn new(base: Rc<Base>) -> Self {
        Self { base }
    }
}
