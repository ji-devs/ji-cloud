use components::module::_common::edit::prelude::*;
use crate::base::state::Base;
use std::rc::Rc;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Signal, Mutable, SignalExt};
use dominator::clone;
use super::{
    step_1::state::Step1,
    step_2::state::Step2,
    step_3::state::Step3,
    step_4::state::Step4,
};

pub struct Sidebar {
    pub base: Rc<Base>,
    pub tab_index: Mutable<Option<usize>>
}


impl Sidebar {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base,
            tab_index: Mutable::new(None)
        }
    }
}

impl SidebarExt for Sidebar {
    type TabIndexSignal = impl Signal<Item = Option<usize>>;

    fn tab_index(&self) -> Self::TabIndexSignal {
        self.tab_index.signal()
    }
}
