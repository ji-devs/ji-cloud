use crate::base::state::Base;
use std::rc::Rc;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{SignalVec, SignalVecExt}
};

pub struct MainDrag {
    pub base: Rc<Base>,
}

impl MainDrag {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        Rc::new(Self {
            base,
        })
    }
}

