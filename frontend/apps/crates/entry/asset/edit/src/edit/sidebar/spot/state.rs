use crate::edit::sidebar::state::{Sidebar as SidebarState, SidebarSpot, SidebarSpotItem};
use crate::edit::sidebar::CourseSpot;
use dominator::clone;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::cell::RefCell;
use std::rc::Rc;
use utils::routes::{AssetEditRoute, CourseEditRoute, JigEditRoute};
use web_sys::HtmlElement;

pub struct SpotState {
    pub spot: Rc<SidebarSpot>,
    pub tried_module_at_cover: Mutable<bool>,
    pub sidebar: Rc<SidebarState>,
    pub index: usize,
    pub drag_target_index: Option<usize>,
    pub total_len: usize,
    pub elem: RefCell<Option<HtmlElement>>,
    pub confirm_delete: Mutable<bool>,
}

impl SpotState {
    pub fn new(
        sidebar: Rc<SidebarState>,
        index: usize,
        drag_target_index: Option<usize>,
        total_len: usize,
        module: Rc<SidebarSpot>,
    ) -> Rc<Self> {
        Rc::new(Self {
            spot: module,
            sidebar,
            index,
            drag_target_index,
            total_len,
            tried_module_at_cover: Mutable::new(false),
            elem: RefCell::new(None),
            confirm_delete: Mutable::new(false),
        })
    }

    pub fn kind_str(&self) -> &'static str {
        // match &*self.module {
        //     None => "",
        //     Some(module) => module.kind().as_str(),
        // }
        match &self.spot.item {
            SidebarSpotItem::Jig(Some(module)) => module.kind.as_str(),
            _ => "",
        }
    }

    pub fn unit_name(&self) -> String {
        match &self.spot.item {
            SidebarSpotItem::Course(Some(spot)) => match &**spot {
                CourseSpot::Cover(_) => "Cover".to_string(),
                CourseSpot::Unit(unit) => unit.display_name.clone(),
            },
            _ => "".to_string(),
        }
    }

    pub fn asset_type(&self) -> &'static str {
        match &self.spot.item {
            SidebarSpotItem::Jig(_) => "jig",
            SidebarSpotItem::Playlist(_) => "playlist",
            SidebarSpotItem::Course(_) => "course",
        }
    }

    pub fn is_last_module(&self) -> bool {
        // self.index < self.total_len - 2 && (&*self.module).is_some()
        self.index < self.total_len - 2 && matches!(&self.spot.item, SidebarSpotItem::Jig(Some(_)))
    }

    pub fn window_state_signal(state: Rc<SpotState>) -> impl Signal<Item = &'static str> {
        state.sidebar.asset_edit_state.route.signal_ref(clone!(state => move |route| {
            match &state.spot.item {
                SidebarSpotItem::Jig(module) => {
                    match module {
                        None => "empty",
                        Some(this_module) => {
                            match route {
                                AssetEditRoute::Jig(_, JigEditRoute::Module(active_module_id)) if active_module_id == &this_module.id => "active",
                                _ => "thumbnail",
                            }
                        }
                    }
                },
                SidebarSpotItem::Playlist(playlist_spot) => {
                    match playlist_spot {
                        None => "empty",
                        Some(_) => "thumbnail",
                    }
                },
                SidebarSpotItem::Course(course_spot) => {
                    match course_spot {
                        None => "unit",
                        Some(item) =>
                            match &**item {
                                CourseSpot::Cover(_) => "thumbnail",
                                CourseSpot::Unit(_) => "unit",
                            },
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
            .asset_edit_state
            .route
            .signal_ref(clone!(state => move|route| {
                match &state.spot.item {
                    SidebarSpotItem::Jig(Some(module)) => {
                        matches!(
                            route,
                            AssetEditRoute::Jig(_, JigEditRoute::Module(module_id)) if module_id == &module.id
                        )
                    }
                    SidebarSpotItem::Course(unit) => {
                            match unit {
                                Some(unit) => {
                                    let id = match &**unit {
                                        CourseSpot::Cover(_) => None,
                                        CourseSpot::Unit(unit) => Some(unit.id),
                                    };

                                    matches!(
                                        route,
                                        AssetEditRoute::Course(_, CourseEditRoute::Unit(unit_id)) if unit_id == &id
                                    )
                                },
                                None => {
                                    // Handle the None case here.
                                    false // for example, you might simply return false
                                },
                            }
                        }
                    _ => {
                        false
                    }
                }
            }))
    }
}
