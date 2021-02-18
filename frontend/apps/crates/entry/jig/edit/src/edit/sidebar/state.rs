use futures_signals::signal_vec::MutableVec;
use futures_signals::signal::Mutable;
use shared::domain::jig::{Jig, LiteModule, JigId, ModuleId, ModuleKind};
use std::rc::Rc;
use super::module::state::Module;

pub struct State {
    pub jig_id: JigId,
    pub name: Mutable<Option<String>>,
    pub modules: MutableVec<Rc<Module>>,
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
            )
        }

    }
}
