use components::module::edit::prelude::*;
use components::traces::{
    bubble::state::TraceBubble,
    edit::state::Edit as TracesEdit
};
use crate::base::state::Base;
use std::rc::Rc;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{SignalVec, SignalVecExt}
};
use utils::prelude::*;
use dominator::clone;

pub struct Main {
    pub base: Rc<Base>,
}

impl Main {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base,
        }
    }

}

impl MainExt for Main {
}


