use crate::data::*;
use crate::debug;
use components::module::page::ModulePageKind;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt}
};

pub struct State {
    pub data: Mutable<Option<raw::GameData>>,
    pub jig_id: String,
    pub module_id: String,
}
impl State {
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
