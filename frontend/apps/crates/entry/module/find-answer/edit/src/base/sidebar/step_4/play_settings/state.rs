use crate::base::state::Base;
use std::rc::Rc;

// pub const DEFAULT_SELECT_AMOUNT: usize = 3;

pub struct State {
    pub base: Rc<Base>,
}

impl State {
    pub fn new(base: Rc<Base>) -> Self {
        /*
        let some_amount = RefCell::new(
            match base.play_settings.next.get_cloned() {
                Next::SelectSome(value) => value,
                _ => DEFAULT_SELECT_AMOUNT
            }
        );
        */

        Self { base }
    }
}
