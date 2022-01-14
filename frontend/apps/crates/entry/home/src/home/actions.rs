use crate::home::search_results::SearchResults;

use super::state::HomePageMode;
use dominator::clone;
use futures::join;

use shared::{
    api::{
        endpoints::jig,
        ApiEndpoint,
    },
    domain::jig::JigCountResponse,
    error::EmptyError
};
use std::rc::Rc;
use utils::prelude::*;

use super::state::State;

pub fn search_url(q: &str) -> String {
    Route::Home(HomeRoute::Search(Some(
        shared::domain::jig::JigSearchQuery {
            q: q.to_string(),
            ..Default::default()
        },
    )))
    .to_string()
}

pub fn fetch_data(state: Rc<State>, include_search: bool) {
    state.loader.load(clone!(state => async move {
        match include_search {
            true => {
                join!(
                    fetch_total_jigs_count(Rc::clone(&state)),
                    fetch_metadata(Rc::clone(&state)),
                    fetch_profile(Rc::clone(&state)),
                    search_async(Rc::clone(&state)),
                );
            },
            false => {
                join!(
                    fetch_total_jigs_count(Rc::clone(&state)),
                    fetch_metadata(Rc::clone(&state)),
                    fetch_profile(Rc::clone(&state)),
                );
            },
        };
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
    match get_user() {
        Some(profile) => {
            state.is_logged_in.set(true);
            state.search_selected.set_from_profile(profile);
        },
        None => {},
    }
}

async fn search_async(state: Rc<State>) {
    let search_state = SearchResults::new(&state, true);
    state.mode.set(HomePageMode::Search(Rc::clone(&search_state)));

    let req = state.search_selected.to_search_request();
    Route::Home(HomeRoute::Search(Some(req.clone()))).push_state();

    join!(
        search_state.jigs.load_items(req.clone()),
        search_state.resources.load_items(req),
    );

    search_state.loading.set(false);
}

pub fn search(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        search_async(state).await;
    }));
}
