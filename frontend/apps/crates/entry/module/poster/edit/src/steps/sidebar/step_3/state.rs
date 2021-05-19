use crate::steps::state::{Step, Base};
use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;

pub struct Step3 {
    pub base: Rc<Base>,
}


impl Step3 {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base,
        }
    }
}
