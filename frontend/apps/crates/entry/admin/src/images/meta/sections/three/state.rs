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

pub struct State {
    pub meta: Rc<MetaState>,
    pub image: Rc<MutableImage>,
    pub metadata: Rc<MetadataResponse>,
    pub categories: Rc<Vec<Rc<MutableCategory>>>,
    pub tag_list: Rc<Vec<(ImageTag, TagId)>>,
}


impl State {
    pub fn new(meta: Rc<MetaState>, image: Rc<MutableImage>, metadata: Rc<MetadataResponse>, categories: Rc<Vec<Rc<MutableCategory>>>, tag_list: Rc<Vec<(ImageTag, TagId)>>) -> Self {
        Self {
            meta,
            image,
            metadata,
            categories,
            tag_list,
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
        let tag_list = self.tag_list.clone();

        self.image.tag_ids.signal_ref(clone!(tag_list => move |ids| {
            ids
                .iter()
                .map(|id| {
                    tag_list.iter().find(|(_, tag_id)| *tag_id == *id)
                        .map(|(tag, _)| tag.STR_DISPLAY_NAME().to_string())
                        .unwrap_or_default()
                })
                .collect::<Vec<String>>()
        }))
        .to_signal_vec()
    }
}


