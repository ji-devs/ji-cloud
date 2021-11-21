use super::state::*;
use dominator::clone;
use std::rc::Rc;
use utils::prelude::*;

impl Puzzle {
    pub fn on_start(self: Rc<Self>) {
        let state = self;

        state.base.allow_stage_click();
    }
}
