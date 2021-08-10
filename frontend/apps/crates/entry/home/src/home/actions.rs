use super::state::HomePageMode;
use dominator::clone;
use futures::join;
use futures_signals::signal_vec::MutableVec;
use shared::{
    api::{
        endpoints::{jig, user::Profile},
        ApiEndpoint,
    },
    domain::{
        jig::{JigCountResponse, JigSearchQuery, JigSearchResponse},
        user::UserProfile,
    },
    error::EmptyError,
};
use std::rc::Rc;
use utils::prelude::*;

use super::state::State;

pub fn fetch_data(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        join!(
            fetch_total_jigs_count(Rc::clone(&state)),
            fetch_metadata(Rc::clone(&state)),
            fetch_profile(Rc::clone(&state)),
        );
    }));
}

async fn fetch_total_jigs_count(state: Rc<State>) {
    match api_no_auth::<JigCountResponse, EmptyError, ()>(
        jig::Count::PATH,
        jig::Count::METHOD,
        None,
    )
    .await
    {
        Err(_) => {}
        Ok(res) => {
            state.total_jigs_count.set(res.total_count);
        }
    };
}

async fn fetch_metadata(state: Rc<State>) {
    state.search_options.populate_options().await;
}

async fn fetch_profile(state: Rc<State>) {
    let (result, status) =
        api_with_auth_status::<UserProfile, EmptyError, ()>(&Profile::PATH, Profile::METHOD, None)
            .await;
    match status {
        403 | 401 => {
            //not logged in
        }
        _ => match result {
            Err(_) => {}
            Ok(profile) => {
                state.is_logged_in.set(true);
                state.search_selected.set_from_profile(&profile);
            }
        },
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
