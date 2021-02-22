use shared::domain::jig::{LiteModule, ModuleId, ModuleKind};
use std::rc::Rc;
use std::cell::RefCell;
use crate::edit::sidebar::state::State as SidebarState;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use utils::drag::Drag;
use web_sys::HtmlElement;
use dominator::clone;

pub struct State {
    pub module: Rc<Module>,
    pub sidebar: Rc<SidebarState>,
    pub drag: Mutable<Option<Drag>>,
    pub index: usize,
    pub total_len: usize,
    pub elem: RefCell<Option<HtmlElement>>
}


impl State {
    pub fn new(sidebar: Rc<SidebarState>, index:usize, total_len: usize, module: Rc<Module>) -> Self {
        Self {
            module,
            sidebar,
            index,
            total_len,
            drag: Mutable::new(None),
            elem: RefCell::new(None),
        }
    }

    pub fn kind_signal(&self) -> impl Signal<Item = Option<ModuleKind>> {
        self.module.kind.signal_cloned()
    }
    pub fn kind_str_signal(&self) -> impl Signal<Item = &'static str> {
        self.kind_signal().map(|kind| {
            match kind {
                Some(kind) => kind.as_str(),
                None => ""
            }
        })
    }
    pub fn window_state_signal(&self) -> impl Signal<Item = &'static str> {
        self.kind_signal().map(|kind| {
            match kind {
                Some(kind) => "draft",
                None => "empty"
            }
        })
    }

    pub fn drag_overlap_signal(_self:Rc<Self>) -> impl Signal<Item = bool> {
        _self.sidebar.drag_target_pos_signal()
            .map(clone!(_self => move |pos| {
                match (pos, _self.elem.borrow().as_ref()) {
                    (Some(pos), Some(elem)) => {
                        let pos_x = pos.x as f64;
                        let pos_y = pos.y as f64 + 100.0;
                        let rect = elem.get_bounding_client_rect();
                        if pos_y > rect.y() && pos_y < (rect.y() + rect.height()) {
                            true
                        } else {
                            false
                        }
                    },
                    _ => false
                }
            }))
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
