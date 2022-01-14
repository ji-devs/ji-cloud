use super::{dragging::state::State as DragState, settings::state::State as SettingsState};
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

pub struct State {
    pub jig: JigResponse,
    pub jig_edit_state: Rc<JigEditState>,
    pub name: Mutable<String>,
    pub publish_at: Mutable<Option<DateTime<Utc>>>,
    pub modules: MutableVec<Rc<Option<LiteModule>>>,
    pub collapsed: Mutable<bool>,
    pub settings: Rc<SettingsState>,
    pub drag: Mutable<Option<Rc<DragState>>>,
    pub drag_target_index: Mutable<Option<usize>>,
    pub loader: AsyncLoader,
}

impl State {
    pub fn new(jig: JigResponse, jig_edit_state: Rc<JigEditState>) -> Self {
        let mut modules: Vec<Rc<Option<LiteModule>>> = jig
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
                    if module.kind != ModuleKind::Cover {
                        modules.insert(0, Rc::new(None));
                    }
                };
            },
        };

        // add empty module at end
        modules.push(Rc::new(None));

        Self {
            jig_edit_state,
            name: Mutable::new(jig.jig_data.display_name.clone()),
            publish_at: Mutable::new(jig.published_at.clone()),
            modules: MutableVec::new_with_values(modules),
            collapsed: Mutable::new(false),
            settings: Rc::new(SettingsState::new(&jig)),
            drag: Mutable::new(None),
            drag_target_index: Mutable::new(None),
            loader: AsyncLoader::new(),
            jig,
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
