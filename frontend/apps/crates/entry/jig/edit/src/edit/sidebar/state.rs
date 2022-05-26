use super::{dragging::state::State as DragState, jig::settings::state::State as SettingsState};
use dominator_helpers::{futures::AsyncLoader, signals::OptionSignal};
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::MutableVec,
};
use shared::domain::{asset::Asset, jig::JigResponse, module::LiteModule};
use std::rc::Rc;
use utils::math::PointI32;

use chrono::{DateTime, Utc};

use super::super::state::State as JigEditState;

/// Determines which window in the sidebar should be highlighted and show an error tooltip
#[derive(Clone, PartialEq)]
pub enum ModuleHighlight {
    /// Module window with the index of the module in the `modules` list
    Module(usize),
    /// Publish window
    Publish,
}

pub struct State {
    pub asset: Asset,
    pub jig_edit_state: Rc<JigEditState>,
    pub name: Mutable<String>,
    pub publish_at: Mutable<Option<DateTime<Utc>>>,
    pub modules: MutableVec<Rc<SidebarSpot>>,
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

impl State {
    pub fn new(jig: Asset, jig_edit_state: Rc<JigEditState>) -> Self {
        let mut modules = match &jig {
            Asset::Jig(jig) => Self::get_jig_spots(jig),
            Asset::Course(_) => todo!(),
        };

        modules.push(Rc::new(SidebarSpot::new_empty(&jig)));

        // Initialize these here so that we can move `jig` into the initialization of Self and
        // still keep the ordering of the fields.
        let jig_display_name = jig.display_name().clone();
        // let jig_published_at = jig.published_at;
        let jig_published_at = None;
        let settings_state = match &jig {
            Asset::Jig(jig) => SidebarSetting::Jig(Rc::new(SettingsState::new(jig))),
            Asset::Course(_) => todo!(),
        };

        Self {
            asset: jig,
            jig_edit_state,
            name: Mutable::new(jig_display_name),
            publish_at: Mutable::new(jig_published_at),
            modules: MutableVec::new_with_values(modules),
            collapsed: Mutable::new(false),
            settings: settings_state,
            drag: Mutable::new(None),
            drag_target_index: Mutable::new(None),
            highlight_modules: Mutable::new(None), // By default we don't want modules highlighted yet.
            loader: AsyncLoader::new(),
        }
    }

    fn get_jig_spots(jig: &JigResponse) -> Vec<Rc<SidebarSpot>> {
        jig.jig_data
            .modules
            .iter()
            .map(|module| Rc::new(module.clone().into()))
            .collect()
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
        let modules = self.modules.lock_ref();

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
    pub fn new_empty(asset: &Asset) -> Self {
        match asset {
            Asset::Jig(_) => Self {
                item: SidebarSpotItem::Jig(None),
                is_incomplete: Mutable::new(false),
            },
            Asset::Course(_) => todo!(),
        }
    }
}

impl From<LiteModule> for SidebarSpot {
    fn from(module: LiteModule) -> Self {
        Self {
            is_incomplete: Mutable::new(!module.is_complete),
            item: SidebarSpotItem::Jig(Some(Rc::new(module))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SidebarSpotItem {
    Jig(Option<Rc<LiteModule>>),
}

impl SidebarSpotItem {
    pub fn is_some(&self) -> bool {
        match self {
            Self::Jig(module) => module.is_some(),
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            Self::Jig(module) => module.is_none(),
        }
    }
    pub fn unwrap_module(&self) -> &Option<Rc<LiteModule>> {
        match self {
            Self::Jig(module) => module,
        }
    }
}

pub(super) enum SidebarSetting {
    Jig(Rc<SettingsState>),
}
