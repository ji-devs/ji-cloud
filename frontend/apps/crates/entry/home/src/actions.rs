use std::rc::Rc;
use dominator::clone;
use futures::join;
use futures_signals::signal_vec::MutableVec;
use shared::{api::{ApiEndpoint, endpoints::{jig, user::Profile}}, domain::{jig::{JigSearchQuery, JigSearchResponse}, user::UserProfile}, error::EmptyError};
use utils::prelude::*;
use crate::state::HomePageMode;

use super::state::State;

pub fn fetch_data(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        join!(
            fetch_metadata(Rc::clone(&state)),
            fetch_profile(Rc::clone(&state)),
        );
    }));
}


async fn fetch_metadata(state: Rc<State>) {
    state.search_options.populate_options().await;

    // if ages are not yet populated from profile set to default
    let mut age_options = state.search_selected.age_ranges.lock_mut();
    if age_options.len() == 0 {
        let default_age = state.search_options.age_ranges.lock_ref()[0].id.clone();
        age_options.insert(default_age);
    };

    // TODO: deal with goal/subject
}

async fn fetch_profile(state: Rc<State>) {
    let (result, status) = api_with_auth_status::<UserProfile, EmptyError, ()>(&Profile::PATH, Profile::METHOD, None).await;
    match status  {
        403 | 401 => {
            //not logged in
        }
        _ => {
            match result {
                Err(_) => {},
                Ok(profile) => {
                    state.is_logged_in.set(true);
                    state.search_selected.set_from_profile(&profile);
                }
            }
        }
    };
}

pub fn search(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let req = state.search_selected.to_search_request();
        let query = req.q.to_owned();
        match api_no_auth::<JigSearchResponse, EmptyError, JigSearchQuery>(jig::Search::PATH, jig::Search::METHOD, Some(req)).await {
            Err(_) => {},
            Ok(res) => {
                let jigs = res.jigs.into_iter().map(|jr| jr.jig).collect();
                state.mode.set(HomePageMode::Search(query, Rc::new(MutableVec::new_with_values(jigs))));
            },
        };
    }));
}
