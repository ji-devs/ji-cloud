use dominator::{html, clone, Dom};
use std::rc::Rc;
use crate::images::meta::state::{State as MetaState, MutableImage};
use super::{state::*, actions};
use utils::{events, routes::*, api_helpers::meta::MetaOptions};
use wasm_bindgen::prelude::*;
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use shared::domain::meta::{MetadataResponse, TagId};
use components::image::tag::ImageTag;

pub struct Section1Dom {
}

impl Section1Dom {
    pub fn render(meta_state: Rc<MetaState>, image: Rc<MutableImage>, metadata: Rc<MetadataResponse>, tag_list: Rc<Vec<(ImageTag, TagId)>>) -> Dom {
        let state = Rc::new(State::new(meta_state, image, metadata, tag_list));

        html!("image-meta-section-1", {
            .children(state.metadata.image_styles.iter().map(|style| {
                let id = style.id;
                html!("input-checkbox", {
                    .property("slot", "styles")
                    .property("label", &style.display_name)
                    .property_signal("checked", state.style_selected(id.clone()))
                    .event(clone!(state, id => move |evt:events::CustomToggle| {
                        actions::toggle_style(state.clone(), id, evt.value());
                    }))
                })
            }))
            .children(state.metadata.age_ranges.iter().map(|age_range| {
                let id = age_range.id;
                html!("input-checkbox", {
                    .property("slot", "age_ranges")
                    .property("label", &age_range.display_name)
                    .property_signal("checked", state.age_range_selected(id.clone()))
                    .event(clone!(state, id => move |evt:events::CustomToggle| {
                        actions::toggle_age_range(state.clone(), id, evt.value());
                    }))
                })
            }))
            .children(state.metadata.affiliations.iter().map(|affiliation| {
                let id = affiliation.id;
                html!("input-checkbox", {
                    .property("slot", "affiliations")
                    .property("label", &affiliation.display_name)
                    .property_signal("checked", state.affiliation_selected(id.clone()))
                    .event(clone!(state, id => move |evt:events::CustomToggle| {
                        actions::toggle_affiliation(state.clone(), id, evt.value());
                    }))
                })
            }))
            .children(state.tag_list.iter().map(|(tag, tag_id)| {
                html!("input-checkbox", {
                    .property("slot", "tags")
                    .property("label", tag.STR_DISPLAY_NAME())
                    .property_signal("checked", state.tag_selected(tag_id.clone()))
                    .event(clone!(state, tag_id => move |evt:events::CustomToggle| {
                        actions::toggle_tag(state.clone(), tag_id, evt.value());
                    }))
                })
            }))
        })
    }
}
