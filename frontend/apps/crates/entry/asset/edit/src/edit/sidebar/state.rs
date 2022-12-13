use super::{
    course::settings::CourseSettings, dragging::state::State as DragState,
    jig::settings::JigSettings,
};
use dominator_helpers::{futures::AsyncLoader, signals::OptionSignal};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::{asset::AssetId, jig::JigResponse, module::LiteModule};
use std::rc::Rc;
use utils::{editable_asset::EditableAsset, math::PointI32};

use super::super::state::AssetEditState;

/// Determines which window in the sidebar should be highlighted and show an error tooltip
#[derive(Clone, PartialEq)]
pub enum ModuleHighlight {
    /// Module window with the index of the module in the `modules` list
    Module(usize),
    /// Publish window
    Publish,
}

pub struct Sidebar {
    pub asset_edit_state: Rc<AssetEditState>,
    // pub spots: MutableVec<Rc<SidebarSpot>>,
    pub collapsed: Mutable<bool>,
    pub drag: Mutable<Option<Rc<DragState>>>,
    pub drag_target_index: Mutable<Option<usize>>,
    /// Whether to highlight incomplete modules. This is useful so that we can _only_ highlight
    /// modules once the teacher performs a specific action, such as clicking "Publish".
    /// Holds the index of the first module in the list which is incomplete
    pub highlight_modules: Mutable<Option<ModuleHighlight>>,
    pub loader: AsyncLoader,
    pub(super) settings: SidebarSetting,
}

impl Sidebar {
    pub fn new(asset_edit_state: Rc<AssetEditState>) -> Rc<Self> {
        let settings_state = match &asset_edit_state.asset {
            EditableAsset::Jig(jig) => SidebarSetting::Jig(JigSettings::new(jig)),
            EditableAsset::Course(course) => SidebarSetting::Course(CourseSettings::new(course)),
            EditableAsset::Resource(_) => {
                unimplemented!()
            }
        };

        Rc::new(Self {
            asset_edit_state,
            // spots: MutableVec::new_with_values(modules),
            collapsed: Mutable::new(false),
            settings: settings_state,
            drag: Mutable::new(None),
            drag_target_index: Mutable::new(None),
            highlight_modules: Mutable::new(None), // By default we don't want modules highlighted yet.
            loader: AsyncLoader::new(),
        })
    }

    //There's probably a way of making this simpler
    //But in any case, the signature is what matters :P
    pub fn drag_target_pos_signal(&self) -> impl Signal<Item = Option<PointI32>> {
        self.drag
            .signal_cloned()
            .map(|drag| OptionSignal::new(drag.map(|drag| drag.inner.pos_signal())))
            .flatten()
            .map(|x| x.and_then(|x| x))
    }

    /// Returns whether this JIG is publishable
    pub fn can_publish(&self) -> bool {
        let modules = self.asset_edit_state.sidebar_spots.lock_ref();

        let modules_len = modules
            .iter()
            .filter(|module| module.item.is_some())
            .count();

        let modules_valid = !modules.iter().any(|module| module.is_incomplete.get());

        modules_len > 0 && modules_valid
    }

    /*
    pub fn bounding_boxes(&self) -> Vec<(usize, DomRect)> {
        self.drag_targets
            .borrow()
            .iter()
            .map(|(index, module)| {
                //This must exist since it's added before the module
                //is added to drag_targets
                let elem = module.elem.borrow();
                let elem = elem.as_ref().unwrap_ji();
                let rect = elem.get_bounding_client_rect();
                (*index, rect)
            })
            .collect()
    }
    */
}

#[derive(Clone, Debug)]
pub struct SidebarSpot {
    pub item: SidebarSpotItem,
    pub is_incomplete: Mutable<bool>,
}

impl SidebarSpot {
    pub fn new_empty(asset_id: &AssetId) -> Self {
        let item = match asset_id {
            AssetId::JigId(_) => SidebarSpotItem::Jig(None),
            AssetId::CourseId(_) => SidebarSpotItem::Course(None),
            AssetId::ResourceId(_) => unimplemented!(),
        };
        Self {
            item,
            is_incomplete: Mutable::new(false),
        }
    }

    pub fn new_jig_module(module: Option<LiteModule>) -> Rc<Self> {
        Rc::new(Self {
            is_incomplete: Mutable::new(match &module {
                Some(module) => !module.is_complete,
                None => false,
            }),
            item: SidebarSpotItem::Jig(module.map(|module| Rc::new(module))),
        })
    }

    pub fn new_course_cover(cover: LiteModule) -> Rc<Self> {
        Rc::new(Self {
            is_incomplete: Mutable::new(false),
            item: SidebarSpotItem::Course(Some(Rc::new(CourseSpot::Cover(cover)))),
        })
    }

    pub fn new_course_item(jig: JigResponse) -> Rc<Self> {
        Rc::new(Self {
            is_incomplete: Mutable::new(false),
            item: SidebarSpotItem::Course(Some(Rc::new(CourseSpot::Item(jig)))),
        })
    }
}

#[derive(Clone, Debug)]
pub enum SidebarSpotItem {
    Jig(Option<Rc<LiteModule>>),
    Course(Option<Rc<CourseSpot>>),
}

impl SidebarSpotItem {
    pub fn is_some(&self) -> bool {
        match self {
            Self::Jig(module) => module.is_some(),
            Self::Course(course_spot) => course_spot.is_some(),
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            Self::Jig(module) => module.is_none(),
            Self::Course(course_spot) => course_spot.is_none(),
        }
    }
    pub fn unwrap_jig(&self) -> &Option<Rc<LiteModule>> {
        match self {
            Self::Jig(module) => module,
            _ => panic!(),
        }
    }
    pub fn unwrap_course(&self) -> &Option<Rc<CourseSpot>> {
        match self {
            Self::Course(course_spot) => course_spot,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CourseSpot {
    Cover(LiteModule),
    Item(JigResponse),
}

// #[derive(Clone, Debug)]
// pub enum CourseSpot {
//     Cover(Option<Rc<LiteModule>>),
//     Item(Option<Rc<JigId>>),
// }

// impl CourseSpot {
//     pub fn is_some(&self) -> bool {
//         match self {
//             Self::Cover(cover) => cover.is_some(),
//             Self::Item(item) => item.is_some(),
//         }
//     }

//     pub fn is_none(&self) -> bool {
//         match self {
//             Self::Cover(cover) => cover.is_none(),
//             Self::Item(item) => item.is_none(),
//         }
//     }
// }

pub(super) enum SidebarSetting {
    Jig(Rc<JigSettings>),
    Course(Rc<CourseSettings>),
}
