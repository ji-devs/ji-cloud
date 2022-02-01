use super::{dragging::state::State as DragState, settings::state::State as SettingsState};
use components::module::_common::prelude::ModuleId;
use dominator_helpers::{futures::AsyncLoader, signals::OptionSignal};
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::MutableVec,
};
use shared::domain::jig::{JigResponse, LiteModule, ModuleKind};
use std::rc::Rc;
use utils::math::PointI32;

use chrono::{DateTime, Utc};

use super::super::state::State as JigEditState;

/// A wrapper struct for LiteModule so that the fields can be used as signals. State::modules is a
/// MutableVec, but replacing the entire LiteModule whenever is_complete changes means that the
/// entire item in the sidebar will be rerendered.
///
/// Should be constructed from a LiteModule.
pub struct Module {
    inner: Rc<LiteModule>,

    /// Whether this module is completed.
    pub is_complete: Mutable<bool>,
}

impl Module {
    pub fn id(&self) -> &ModuleId {
        &self.inner.id
    }

    pub fn kind(&self) -> &ModuleKind {
        &self.inner.kind
    }
}

impl From<LiteModule> for Module {
    fn from(module: LiteModule) -> Self {
        let is_complete = Mutable::new(module.is_complete);

        Self {
            inner: Rc::new(module),
            is_complete,
        }
    }
}

impl From<&Module> for LiteModule {
    fn from(module: &Module) -> LiteModule {
        LiteModule {
            id: module.inner.id,
            kind: module.inner.kind,
            is_complete: module.is_complete.get(),
        }
    }
}

/// Determines which window in the sidebar should be highlighted and show an error tooltip
#[derive(Clone, PartialEq)]
pub enum ModuleHighlight {
    /// Module window with the index of the module in the `modules` list
    Module(usize),
    /// Publish window
    Publish,
}

pub struct State {
    pub jig: JigResponse,
    pub jig_edit_state: Rc<JigEditState>,
    pub name: Mutable<String>,
    pub publish_at: Mutable<Option<DateTime<Utc>>>,
    pub modules: MutableVec<Rc<Option<Module>>>,
    pub collapsed: Mutable<bool>,
    pub settings: Rc<SettingsState>,
    pub drag: Mutable<Option<Rc<DragState>>>,
    pub drag_target_index: Mutable<Option<usize>>,
    /// Whether to highlight incomplete modules. This is useful so that we can _only_ highlight
    /// modules once the teacher performs a specific action, such as clicking "Publish".
    /// Holds the index of the first module in the list which is incomplete
    pub highlight_modules: Mutable<Option<ModuleHighlight>>,
    pub loader: AsyncLoader,
}

impl State {
    pub fn new(jig: JigResponse, jig_edit_state: Rc<JigEditState>) -> Self {
        let mut modules: Vec<Rc<Option<Module>>> = jig
            .jig_data
            .modules
            .iter()
            .map(|module| Rc::new(Some(module.clone().into())))
            .collect();

        match modules.get(0) {
            // If there are no modules, add a default placeholder which will be used for setting
            // a cover on the JIG.
            None => {
                modules.push(Rc::new(None));
            },
            // If the first module is not a cover module, insert a placeholder module before that
            // so that a cover can still be set on the JIG.
            Some(module) => {
                if let Some(module) = &**module {
                    if *module.kind() != ModuleKind::Cover {
                        modules.insert(0, Rc::new(None));
                    }
                };
            },
        };

        // add empty module at end
        modules.push(Rc::new(None));

        // Initialize these here so that we can move `jig` into the initialization of Self and
        // still keep the ordering of the fields.
        let jig_display_name = jig.jig_data.display_name.clone();
        let jig_published_at = jig.published_at.clone();
        let settings_state = SettingsState::new(&jig);

        Self {
            jig,
            jig_edit_state,
            name: Mutable::new(jig_display_name),
            publish_at: Mutable::new(jig_published_at),
            modules: MutableVec::new_with_values(modules),
            collapsed: Mutable::new(false),
            settings: Rc::new(settings_state),
            drag: Mutable::new(None),
            drag_target_index: Mutable::new(None),
            highlight_modules: Mutable::new(None), // By default we don't want modules highlighted yet.
            loader: AsyncLoader::new(),
        }
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

        let modules_len = modules.iter().filter(|module| module.is_some()).count();

        let modules_valid = modules.into_iter().find(|module| {
            match &***module {
                // Find the first module which isn't complete
                Some(module) => !module.is_complete.get_cloned(),
                None => false,
            }
        })
        .is_none();

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
