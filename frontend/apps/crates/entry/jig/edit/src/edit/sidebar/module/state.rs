use shared::domain::jig::{LiteModule, ModuleId, ModuleKind};
use std::rc::Rc;
use crate::edit::sidebar::state::State as SidebarState;
use futures_signals::signal::{Mutable, Signal, SignalExt};

pub struct State {
    pub module: Rc<Module>,
    pub sidebar: Rc<SidebarState>,
    pub index: usize,
    pub total_len: usize,
}

impl State {
    pub fn new(sidebar: Rc<SidebarState>, index:usize, total_len: usize, module: Rc<Module>) -> Self {
        Self {
            module,
            sidebar,
            index,
            total_len,
        }
    }

    pub fn kind_signal(&self) -> impl Signal<Item = Option<ModuleKind>> {
        self.module.kind.signal_cloned()
    }
}

pub struct Module {
    pub id: ModuleId,
    pub kind: Mutable<Option<ModuleKind>>,
}

impl Module {
    pub fn new(id: ModuleId) -> Self {
        Self {
            id,
            kind: Mutable::new(None),
        }
    }
}
impl From<LiteModule> for Module {
    fn from(raw:LiteModule) -> Self {
        Self {
            id: raw.id,
            kind: Mutable::new(raw.kind),
        }
    }
}
