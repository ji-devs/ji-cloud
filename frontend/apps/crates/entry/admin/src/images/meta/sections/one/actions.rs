use shared::{
    api::{ApiEndpoint, endpoints},
    domain::image::*,
    domain::meta::*,
    error::EmptyError,
};
use utils::fetch::{api_no_auth, api_with_auth, api_with_auth_empty};
use dominator::clone;
use super::state::*;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

pub fn toggle_style(state: Rc<State>, id: StyleId, flag: bool) {
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
            styles: Some(state.image.styles.lock_ref().iter().map(|x| x.clone()).collect()),
            ..ImageUpdateRequest::default()
        }
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
            age_ranges: Some(state.image.age_ranges.lock_ref().iter().map(|x| x.clone()).collect()),
            ..ImageUpdateRequest::default()
        }
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
            affiliations: Some(state.image.affiliations.lock_ref().iter().map(|x| x.clone()).collect()),
            ..ImageUpdateRequest::default()
        }
    );
}
