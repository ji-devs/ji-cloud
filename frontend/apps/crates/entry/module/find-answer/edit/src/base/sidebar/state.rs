use crate::base::state::Base;
use components::{module::_common::edit::prelude::*, tabs::MenuTabKind};
use std::rc::Rc;

use futures_signals::signal::{Mutable, Signal};

pub struct Sidebar {
    pub base: Rc<Base>,
    pub tab_kind: Mutable<Option<MenuTabKind>>,
}

impl Sidebar {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base,
            tab_kind: Mutable::new(None),
        }
    }
}

impl SidebarExt for Sidebar {
    type TabKindSignal = impl Signal<Item = Option<MenuTabKind>>;

    fn tab_kind(&self) -> Self::TabKindSignal {
        self.tab_kind.signal()
    }
}
