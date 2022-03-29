use crate::edit::sidebar::state::{Module, State as SidebarState};
use dominator::clone;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::cell::RefCell;
use std::rc::Rc;
use utils::drag::Drag;
use utils::routes::JigEditRoute;
use web_sys::HtmlElement;

pub struct State {
    pub module: Rc<Option<Module>>,
    pub tried_module_at_cover: Mutable<bool>,
    pub sidebar: Rc<SidebarState>,
    pub drag: Mutable<Option<Drag>>,
    pub index: usize,
    pub total_len: usize,
    pub elem: RefCell<Option<HtmlElement>>,
    pub confirm_delete: Mutable<bool>,
}

impl State {
    pub fn new(
        sidebar: Rc<SidebarState>,
        index: usize,
        total_len: usize,
        module: Rc<Option<Module>>,
    ) -> Self {
        Self {
            module,
            sidebar,
            index,
            total_len,
            tried_module_at_cover: Mutable::new(false),
            drag: Mutable::new(None),
            elem: RefCell::new(None),
            confirm_delete: Mutable::new(false),
        }
    }

    pub fn kind_str(&self) -> &'static str {
        match &*self.module {
            None => "",
            Some(module) => module.kind().as_str(),
        }
    }

    pub fn is_last_module(&self) -> bool {
        self.index < self.total_len - 2 && (&*self.module).is_some()
    }

    pub fn window_state_signal(state: Rc<State>) -> impl Signal<Item = &'static str> {
        state.sidebar.jig_edit_state.route.signal_ref(clone!(state => move |route| {
            match &*state.module {
                None => return "empty",
                Some(this_module) => {
                    match route {
                        JigEditRoute::Module(active_module_id) if active_module_id == this_module.id() => return "active",
                        _ => return "thumbnail",
                    }
                }
            };
        }))
    }

    pub fn drag_overlap_signal(_self: Rc<Self>) -> impl Signal<Item = bool> {
        _self
            .sidebar
            .drag_target_pos_signal()
            .map(clone!(_self => move |pos| {
                match (pos, _self.elem.borrow().as_ref()) {
                    (Some(pos), Some(elem)) => {
                        let _pos_x = pos.x as f64;
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
