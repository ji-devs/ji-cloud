use futures_signals::signal_vec::MutableVec;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::jig::{Jig, LiteModule, JigId, ModuleId, ModuleKind};
use std::rc::Rc;
use super::module::state::{Module, State as ModuleState};
use utils::drag::Drag;
use super::dragging::state::State as DragState;
use dominator_helpers::signals::OptionSignal;

pub struct State {
    pub jig_id: JigId,
    pub name: Mutable<Option<String>>,
    pub modules: MutableVec<Rc<Module>>,
    pub drag: Mutable<Option<Rc<DragState>>>,
}

impl State {
    pub fn new(jig:Jig) -> Self {
        Self {
            jig_id: jig.id,
            name: Mutable::new(jig.display_name),
            modules: MutableVec::new_with_values(
                jig.modules
                    .into_iter()
                    .map(|module| Rc::new(module.into()))
                    .collect()
            ),
            drag: Mutable::new(None) 
        }

    }

    //There's probably a way of making this simpler
    //But in any case, the signature is what matters :P
    pub fn drag_target_index_signal(&self) -> impl Signal<Item = Option<usize>> {
        self.drag.signal_cloned().map(|drag| {
            OptionSignal::new(
                drag.map(|drag| drag.target_index.signal())
            )
        })
        .flatten()
        .map(|x| x.and_then(|x| x))
    }
}

