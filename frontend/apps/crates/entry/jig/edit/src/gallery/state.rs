use shared::domain::jig::JigId;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
pub struct State {
    pub loader: AsyncLoader,
    pub jigs: MutableVec<(JigId, Option<String>)> 
}

impl State {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            jigs: MutableVec::new()
        }
    }

    //is loaded and number of children
    pub fn loaded_signal(&self) -> impl Signal<Item = (bool, usize)> {
        map_ref! {
            let is_loading = self.loader.is_loading(),
            let n_loaded = self.jigs.signal_vec_cloned().len()
                => {
                    (*is_loading, *n_loaded)
                }
        }
    }
}
