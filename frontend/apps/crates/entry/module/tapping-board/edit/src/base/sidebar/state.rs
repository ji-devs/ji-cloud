use crate::base::state::Base;
use components::module::_common::edit::prelude::*;
use std::rc::Rc;

use futures_signals::signal::{Mutable, Signal};

pub struct Sidebar {
    pub base: Rc<Base>,
    pub tab_index: Mutable<Option<usize>>,
}

impl Sidebar {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base,
            tab_index: Mutable::new(None),
        }
    }
}

impl SidebarExt for Sidebar {
    type TabIndexSignal = impl Signal<Item = Option<usize>>;

    fn tab_index(&self) -> Self::TabIndexSignal {
        self.tab_index.signal()
    }
}
