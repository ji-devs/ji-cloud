use components::module::page::ModulePageKind;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt}
};
use std::rc::Rc;
use crate::{
    debug,
    data::{*, raw::GameData as RawData},
};
use std::future::Future;
use components::module::page::StateLoader;


pub struct LocalState {
    pub data: Mutable<Option<raw::GameData>>,
    pub jig_id: String,
    pub module_id: String,
}
impl LocalState {
    pub fn new(jig_id: String, module_id: String, data: Option<raw::GameData>) -> Self {
        Self { 
            data: Mutable::new(data),
            jig_id,
            module_id,
        }
    }

    pub fn page_kind_signal(&self) -> impl Signal<Item = ModulePageKind> {
        self.data.signal_ref(|data| {
            if data.is_some() {
                ModulePageKind::GridResize
            } else {
                ModulePageKind::GridPlain
            }
        })
    }
}

pub struct PageLoader { 
    pub jig_id: String,
    pub module_id: String
}
impl StateLoader<RawData, LocalState> for PageLoader {
    type FutureState = impl Future<Output = Option<Rc<LocalState>>>;
    fn load_state(&self) -> Self::FutureState{ 
        let jig_id = self.jig_id.clone();
        let module_id = self.module_id.clone();
        async move {
            let game_data = debug::settings().data;
            let state = Rc::new(LocalState::new(jig_id, module_id, game_data));
            Some(state)
        }
    }

    fn derive_state(&self, data:RawData) -> Rc<LocalState> { 
        Rc::new(LocalState::new(self.jig_id.clone(), self.module_id.clone(), Some(data)))
    }
}
