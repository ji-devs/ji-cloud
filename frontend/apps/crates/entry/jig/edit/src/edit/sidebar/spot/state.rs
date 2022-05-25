use crate::edit::sidebar::state::{SidebarSpot, SidebarSpotItem, State as SidebarState};
use dominator::clone;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::cell::RefCell;
use std::rc::Rc;
use utils::drag::Drag;
use utils::routes::{JigEditRoute, AssetEditRoute};
use web_sys::HtmlElement;

pub struct State {
    pub module: Rc<SidebarSpot>,
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
        module: Rc<SidebarSpot>,
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
        // match &*self.module {
        //     None => "",
        //     Some(module) => module.kind().as_str(),
        // }
        match &self.module.item {
            SidebarSpotItem::Jig(Some(module)) => module.kind.as_str(),
            _ => "",
        }
    }

    pub fn is_last_module(&self) -> bool {
        // self.index < self.total_len - 2 && (&*self.module).is_some()
        self.index < self.total_len - 2
            && matches!(&self.module.item, SidebarSpotItem::Jig(Some(_)))
    }

    pub fn window_state_signal(state: Rc<State>) -> impl Signal<Item = &'static str> {
        state.sidebar.jig_edit_state.route.signal_ref(clone!(state => move |route| {
            match &state.module.item {
                SidebarSpotItem::Jig(module) => {
                    match module {
                        None => "empty",
                        Some(this_module) => {
                            match route {
                                AssetEditRoute::Jig(_, _, JigEditRoute::Module(active_module_id)) if active_module_id == &this_module.id => "active",
                                _ => "thumbnail",
                            }
                        }
                    }
                },
            }
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
                        pos_y > rect.y() && pos_y < (rect.y() + rect.height())
                    },
                    _ => false
                }
            }))
    }

    pub fn is_selected_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        let state = Rc::clone(self);
        state
            .sidebar
            .jig_edit_state
            .route
            .signal_ref(clone!(state => move|route| {
                match &state.module.item {
                    SidebarSpotItem::Jig(Some(module)) => {
                        matches!(
                            route,
                            AssetEditRoute::Jig(_, _, JigEditRoute::Module(module_id)) if module_id == &module.id
                        )
                    }
                    _ => false
                }
            }))
    }
}
