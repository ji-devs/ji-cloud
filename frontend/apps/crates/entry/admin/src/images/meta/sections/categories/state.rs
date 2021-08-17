use crate::images::meta::{
    state::{State as MetaState, MutableImage},
    sections::common::categories::MutableCategory
};
use std::{collections::HashSet, rc::Rc};
use futures_signals::{
    map_ref,
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    signal::{Mutable, Signal, SignalExt}
};
use shared::domain::category::*;
use dominator::clone;

pub struct State {
    pub meta: Rc<MetaState>,
    pub image: Rc<MutableImage>,
    pub categories: Rc<Vec<Rc<MutableCategory>>>,
}


impl State {
    pub fn new(meta: Rc<MetaState>, image: Rc<MutableImage>, categories: Rc<Vec<Rc<MutableCategory>>>) -> Self {
        Self {
            meta,
            image,
            categories,
        }
    }
}


