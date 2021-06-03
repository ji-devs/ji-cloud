use components::module::edit::*;
use crate::steps::state::{Step, Base};
use std::rc::Rc;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;
use super::{
    step_1::state::Step1,
    step_2::state::Step2,
    step_3::state::Step3,
};

pub struct Sidebar {
    pub base: Rc<Base>,
}


impl Sidebar {
    pub fn new(base: Rc<Base>) -> Self {

        Self {
            base,
        }
    }
}

impl SidebarExt for Sidebar {
}
