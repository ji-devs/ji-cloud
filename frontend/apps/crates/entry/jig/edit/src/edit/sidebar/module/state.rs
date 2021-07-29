use shared::domain::jig::{LiteModule, module::ModuleId, ModuleKind};
use utils::routes::JigEditRoute;
use std::rc::Rc;
use std::cell::RefCell;
use crate::edit::sidebar::state::State as SidebarState;
use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};
use utils::drag::Drag;
use web_sys::HtmlElement;
use dominator::clone;

pub struct State {
    pub module: Rc<Option<LiteModule>>,
    pub sidebar: Rc<SidebarState>,
    pub drag: Mutable<Option<Drag>>,
    pub index: usize,
    pub total_len: usize,
    pub elem: RefCell<Option<HtmlElement>>
}


impl State {
    pub fn new(sidebar: Rc<SidebarState>, index:usize, total_len: usize, module: Rc<Option<LiteModule>>) -> Self {
        Self {
            module,
            sidebar,
            index,
            total_len,
            drag: Mutable::new(None),
            elem: RefCell::new(None),
        }
    }

    pub fn kind_str(&self) -> &'static str {
        match &*self.module {
            None => "",
            Some(module) => module.kind.as_str(),
        }
    }
    pub fn window_state_signal(state: Rc<State>) -> impl Signal<Item = &'static str> {
        // TODO: add done state
        map_ref! {
            let publish_at = state.sidebar.publish_at.signal_cloned(),
            let route = state.sidebar.route.signal_cloned()
                => move {
                    if publish_at.is_some() {
                        return "published";
                    };
                    match &*state.module {
                        None => return "empty",
                        Some(this_module) => {
                            match route {
                                JigEditRoute::Module(module_id) if *module_id == this_module.id => return "active",
                                _ => return "draft",
                            }
                        }
                    };
                }
        }
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
