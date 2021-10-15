use crate::images::meta::state::{MutableImage, State as MetaState};
use futures_signals::signal::Signal;
use shared::domain::meta::*;
use std::rc::Rc;

pub struct State {
    pub meta: Rc<MetaState>,
    pub image: Rc<MutableImage>,
    pub metadata: Rc<MetadataResponse>,
}

impl State {
    pub fn new(
        meta: Rc<MetaState>,
        image: Rc<MutableImage>,
        metadata: Rc<MetadataResponse>,
    ) -> Self {
        Self {
            meta,
            image,
            metadata,
        }
    }

    pub fn style_selected(&self, id: ImageStyleId) -> impl Signal<Item = bool> {
        self.image
            .styles
            .signal_ref(move |styles| styles.contains(&id))
    }

    pub fn age_range_selected(&self, id: AgeRangeId) -> impl Signal<Item = bool> {
        self.image
            .age_ranges
            .signal_ref(move |age_ranges| age_ranges.contains(&id))
    }
    pub fn affiliation_selected(&self, id: AffiliationId) -> impl Signal<Item = bool> {
        self.image
            .affiliations
            .signal_ref(move |affiliations| affiliations.contains(&id))
    }

    pub fn tag_selected(&self, tag_index: i16) -> impl Signal<Item = bool> {
        self.image
            .tag_indices
            .signal_ref(move |tag_indices| tag_indices.contains(&tag_index))
    }
}
