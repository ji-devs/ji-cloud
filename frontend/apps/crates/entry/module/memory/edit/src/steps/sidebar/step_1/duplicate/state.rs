use crate::data::*;
use std::rc::Rc;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};

pub struct LocalState {
    pub game_state: Rc<State>,
    pub list: Rc<MutableVec<Mutable<String>>>,
}

impl LocalState {
    pub fn new(game_state: Rc<State>) -> Self {
        Self {
            game_state,
            list: Rc::new(MutableVec::new_with_values(
                    (0..14)
                        .map(|_| Mutable::new(String::default()))
                        .collect()
            ))
        }
    }
}
