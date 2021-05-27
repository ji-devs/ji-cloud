use crate::images::meta::state::{State as MetaState, MutableImage};
use std::{collections::HashSet, rc::Rc};
use futures_signals::{
    map_ref,
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    signal::{Mutable, Signal, SignalExt}
};
use shared::domain::meta::*;
use dominator::clone;

pub struct State {
    pub meta: Rc<MetaState>,
    pub image: Rc<MutableImage>,
    pub metadata: Rc<MetadataResponse>
}


impl State {
    pub fn new(meta: Rc<MetaState>, image: Rc<MutableImage>, metadata: Rc<MetadataResponse>) -> Self {
        Self {
            meta,
            image,
            metadata
        }
    }

    pub fn style_selected(&self, id: ImageStyleId) -> impl Signal<Item = bool> {
        self.image.styles.signal_ref(move |styles| styles.contains(&id))
    }

    pub fn age_range_selected(&self, id: AgeRangeId) -> impl Signal<Item = bool> {
        self.image.age_ranges.signal_ref(move |age_ranges| age_ranges.contains(&id))
    }
    pub fn affiliation_selected(&self, id: AffiliationId) -> impl Signal<Item = bool> {
        self.image.affiliations.signal_ref(move |affiliations| affiliations.contains(&id))
    }
}
