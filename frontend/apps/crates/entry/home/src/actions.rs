use std::rc::Rc;
use dominator::clone;
use futures_signals::signal_vec::MutableVec;
use shared::{api::{ApiEndpoint, endpoints::jig}, domain::{jig::{JigSearchQuery, JigSearchResponse}}, error::EmptyError};
use utils::prelude::*;
use crate::state::HomePageMode;

use super::state::State;

pub fn fetch_metadata(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        state.search_options.populate_options().await;
        let default_age = state.search_options.age_ranges.lock_ref()[0].id.clone();
        log::info!("{:?}", default_age);
        state.search_selected.age_ranges.lock_mut().insert(default_age);
    }));
}


pub fn search(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let req = state.search_selected.to_search_request();
        let query = req.q.to_owned();
        match api_with_auth::<JigSearchResponse, EmptyError, JigSearchQuery>(jig::Search::PATH, jig::Search::METHOD, Some(req)).await {
            Err(_) => {},
            Ok(res) => {
                let jigs = res.jigs.into_iter().map(|jr| jr.jig).collect();
                log::info!("{:?}", jigs);
                state.mode.set(HomePageMode::Search(query, Rc::new(MutableVec::new_with_values(jigs))));
            },
        };
    }));
}
