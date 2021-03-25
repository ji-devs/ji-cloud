use components::module::page::ModulePageKind;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt}
};
use std::rc::Rc;
use crate::{
    debug,
    data::{raw, state::{Step, State}, raw::GameData as RawData},
};
use std::future::Future;
use components::module::page::StateLoader;



pub struct PageLoader { 
    pub jig_id: String,
    pub module_id: String
}
impl StateLoader<RawData, State> for PageLoader {
    type FutureState = impl Future<Output = Option<Rc<State>>>;

    fn load_state(&self) -> Self::FutureState { 
        let jig_id = self.jig_id.clone();
        let module_id = self.module_id.clone();
        async move {
            let game_data = debug::settings().data;
            let state = Rc::new(State::new(jig_id, module_id, game_data));
            Some(state)
        }
    }

    fn derive_state(&self, data:RawData) -> Rc<State> { 
        Rc::new(State::new(self.jig_id.clone(), self.module_id.clone(), Some(data)))
    }
}
