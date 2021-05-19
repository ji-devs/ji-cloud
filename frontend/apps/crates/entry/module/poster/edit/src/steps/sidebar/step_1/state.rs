use crate::steps::state::{Step, Base};
use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;

pub struct Step1 {
    pub base: Rc<Base>,
}


impl Step1 {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base,
        }
    }
}
