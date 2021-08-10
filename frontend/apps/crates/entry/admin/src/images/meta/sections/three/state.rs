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
use shared::domain::meta::*;
use dominator::{html, clone, Dom};
use components::image::tag::ImageTag;
use strum::IntoEnumIterator;

pub struct State {
    pub meta: Rc<MetaState>,
    pub image: Rc<MutableImage>,
    pub metadata: Rc<MetadataResponse>,
    pub categories: Rc<Vec<Rc<MutableCategory>>>,
}


impl State {
    pub fn new(meta: Rc<MetaState>, image: Rc<MutableImage>, metadata: Rc<MetadataResponse>, categories: Rc<Vec<Rc<MutableCategory>>>) -> Self {
        Self {
            meta,
            image,
            metadata,
            categories,
        }
    }

    pub fn styles(&self) -> impl SignalVec<Item = String>  {
        let metadata = self.metadata.clone();
        self.image.styles.signal_ref(clone!(metadata => move |ids| {
            ids
                .iter()
                .map(|id| {
                    metadata.image_styles.iter().find(|hit| hit.id == *id)
                        .map(|hit| hit.display_name.to_string())
                        .unwrap_or_default()
                })
                .collect::<Vec<String>>()
        }))
        .to_signal_vec()
    }
    pub fn age_ranges(&self) -> impl SignalVec<Item = String>  {
        let metadata = self.metadata.clone();
        self.image.age_ranges.signal_ref(clone!(metadata => move |ids| {
            ids
                .iter()
                .map(|id| {
                    metadata.age_ranges.iter().find(|hit| hit.id == *id)
                        .map(|hit| hit.display_name.to_string())
                        .unwrap_or_default()
                })
                .collect::<Vec<String>>()
        }))
        .to_signal_vec()
    }

    pub fn affiliations(&self) -> impl SignalVec<Item = String>  {
        let metadata = self.metadata.clone();
        self.image.affiliations.signal_ref(clone!(metadata => move |ids| {
            ids
                .iter()
                .map(|id| {
                    metadata.affiliations.iter().find(|hit| hit.id == *id)
                        .map(|hit| hit.display_name.to_string())
                        .unwrap_or_default()
                })
                .collect::<Vec<String>>()
        }))
        .to_signal_vec()
    }

    pub fn tags(&self) -> impl SignalVec<Item = String>  {
        self.image.tag_indices.signal_ref(|tag_indices| {
            tag_indices
                .iter()
                .map(|tag_index| {
                    ImageTag::from(*tag_index).STR_DISPLAY_NAME().to_string()
                })
                .collect::<Vec<String>>()
        })
        .to_signal_vec()
    }
}


