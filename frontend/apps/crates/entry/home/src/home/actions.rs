use crate::home::search_results::SearchResults;

use super::state::HomePageMode;
use dominator::clone;
use futures::join;

use shared::{api::endpoints::jig, domain::jig::JigCountPath};
use std::rc::Rc;
use utils::prelude::*;

use super::state::Home;

// pub fn search_url(q: &str) -> String {
//     Route::Home(HomeRoute::Search(Some(
//         shared::domain::jig::JigSearchQuery {
//             q: q.to_string(),
//             ..Default::default()
//         },
//     )))
//     .to_string()
// }

pub fn fetch_data(state: Rc<Home>, include_search: bool) {
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

async fn fetch_total_jigs_count(state: Rc<Home>) {
    match jig::Count::api_no_auth(JigCountPath(), None).await {
        Err(_) => {}
        Ok(res) => {
            state.total_assets_count.set(res.total_count);
        }
    };
}

async fn fetch_metadata(state: Rc<Home>) {
    state.search_options.populate_options().await;
}

async fn fetch_profile(state: Rc<Home>) {
    match get_user_cloned() {
        Some(profile) => {
            state.is_logged_in.set(true);
            state.search_selected.set_from_profile(&profile);
        }
        None => {}
    }
}

async fn search_async(state: Rc<Home>) {
    let search_state = SearchResults::new(&state, true);
    state
        .mode
        .set(HomePageMode::Search(Rc::clone(&search_state)));

    let query_params = state.search_selected.to_query_params();
    Route::Home(HomeRoute::Search(Some(Box::new(query_params)))).push_state();

    join!(
        search_state.jigs.load_items(),
        search_state.resources.load_items(),
        search_state.courses.load_items(),
    );

    search_state.loading.set(false);
}

pub fn search(state: Rc<Home>) {
    state.loader.load(clone!(state => async move {
        search_async(state).await;
    }));
}
