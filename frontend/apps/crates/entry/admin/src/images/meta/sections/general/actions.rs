use shared::{domain::image::*, domain::meta::*};

use super::state::*;
use std::rc::Rc;

pub fn toggle_style(state: Rc<State>, id: ImageStyleId, flag: bool) {
    {
        let mut styles = state.image.styles.lock_mut();
        if flag {
            styles.insert(id);
        } else {
            styles.remove(&id);
        }
    }

    crate::images::meta::actions::save(
        state.meta.clone(),
        ImageUpdateRequest {
            styles: Some(state.image.styles.lock_ref().iter().copied().collect()),
            ..ImageUpdateRequest::default()
        },
    );
}

pub fn toggle_age_range(state: Rc<State>, id: AgeRangeId, flag: bool) {
    {
        let mut age_ranges = state.image.age_ranges.lock_mut();
        if flag {
            age_ranges.insert(id);
        } else {
            age_ranges.remove(&id);
        }
    }

    crate::images::meta::actions::save(
        state.meta.clone(),
        ImageUpdateRequest {
            age_ranges: Some(state.image.age_ranges.lock_ref().iter().copied().collect()),
            ..ImageUpdateRequest::default()
        },
    );
}

pub fn toggle_affiliation(state: Rc<State>, id: AffiliationId, flag: bool) {
    {
        let mut affiliations = state.image.affiliations.lock_mut();
        if flag {
            affiliations.insert(id);
        } else {
            affiliations.remove(&id);
        }
    }

    crate::images::meta::actions::save(
        state.meta.clone(),
        ImageUpdateRequest {
            affiliations: Some(
                state
                    .image
                    .affiliations
                    .lock_ref()
                    .iter()
                    .copied()
                    .collect(),
            ),
            ..ImageUpdateRequest::default()
        },
    );
}

pub fn toggle_tag(state: Rc<State>, tag_index: i16, flag: bool) {
    {
        let mut tag_indices = state.image.tag_indices.lock_mut();
        if flag {
            tag_indices.insert(tag_index);
        } else {
            tag_indices.remove(&tag_index);
        }
    }

    crate::images::meta::actions::save(
        state.meta.clone(),
        ImageUpdateRequest {
            tags: Some(
                state
                    .image
                    .tag_indices
                    .lock_ref()
                    .iter()
                    .map(|x| ImageTagIndex(*x))
                    .collect(),
            ),
            ..ImageUpdateRequest::default()
        },
    );
}
