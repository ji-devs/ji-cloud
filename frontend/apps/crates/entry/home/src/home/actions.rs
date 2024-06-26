use crate::home::search_results::SearchResults;

use super::state::HomePageMode;
use dominator::clone;
use futures::join;

use shared::{
    api::endpoints,
    domain::jig::{JigFeaturedPath, JigSearchPath, JigTrendingPath, ListLikedPath, ListPlayedPath},
};
use std::{collections::HashMap, rc::Rc};
use utils::{bail_on_err, init::analytics, metadata::get_age_ranges, prelude::*};

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

pub fn fetch_data(state: Rc<Home>, is_search: bool) {
    state.loader.load(clone!(state => async move {
        match is_search {
            true => {
                join!(
                    fetch_total_jigs_count(Rc::clone(&state)),
                    fetch_profile(Rc::clone(&state)),
                    search_async(Rc::clone(&state)),
                );
            },
            false => {
                join!(
                    fetch_total_jigs_count(Rc::clone(&state)),
                    fetch_profile(Rc::clone(&state)),
                    fetch_trending(Rc::clone(&state)),
                    fetch_featured(Rc::clone(&state)),
                );
                if is_user_set() {
                    fetch_liked(Rc::clone(&state)).await;
                    fetch_played(Rc::clone(&state)).await;
                }
            },
        };
    }));
}

async fn fetch_total_jigs_count(state: Rc<Home>) {
    // using search instead of count api because of differences in count result
    // match jig::Count::api_no_auth(JigCountPath(), None).await {
    match endpoints::jig::Search::api_no_auth(JigSearchPath(), None).await {
        Err(_) => {}
        Ok(res) => {
            state.total_assets_count.set(res.total_jig_count);
        }
    };
}

async fn fetch_profile(state: Rc<Home>) {
    match get_user_cloned() {
        Some(profile) => {
            state.is_logged_in.set(true);
            state.search_bar.search_selected.set_from_profile(&profile);
        }
        None => {}
    }
}

async fn fetch_trending(state: Rc<Home>) {
    let res = endpoints::jig::Trending::api_with_auth(JigTrendingPath(), None)
        .await
        .toast_on_err();
    let res = bail_on_err!(res);
    state.trending.set(Some(res.jigs));
}

async fn fetch_featured(state: Rc<Home>) {
    let res = endpoints::jig::Featured::api_with_auth(JigFeaturedPath(), None)
        .await
        .toast_on_err();
    let res = bail_on_err!(res);
    state.featured.set(Some(res.jigs));
}

async fn fetch_liked(state: Rc<Home>) {
    let res = endpoints::jig::ListLiked::api_with_auth(ListLikedPath(), None)
        .await
        .toast_on_err();
    let res = bail_on_err!(res);
    state.liked.set(Some(res.jigs));
}

async fn fetch_played(state: Rc<Home>) {
    let res = endpoints::jig::ListPlayed::api_with_auth(ListPlayedPath(), None)
        .await
        .toast_on_err();
    let res = bail_on_err!(res);
    state.played.set(Some(res.jigs));
}

async fn search_async(state: Rc<Home>) {
    let search_state = SearchResults::new(&state, true);
    state
        .mode
        .set(HomePageMode::Search(Rc::clone(&search_state)));

    let query_params = state.search_bar.search_selected.to_query_params();
    Route::Home(HomeRoute::Search(Some(Box::new(query_params.clone())))).push_state();

    join!(
        search_state.jigs.load_items(),
        search_state.resources.load_items(),
        search_state.playlists.load_items(),
    );

    search_state.loading.set(false);

    // Analytics event tracking
    let age_ranges = get_age_ranges().await;
    let mut properties = HashMap::new();
    let ages = query_params
        .age_ranges
        .iter()
        .map(|v| {
            let age = age_ranges.iter().find(|age| age.id == *v).unwrap_ji();
            age.display_name.clone()
        })
        .collect::<Vec<String>>()
        .join(",");
    let language = query_params.language.unwrap_or("All languages".to_owned());
    properties.insert("Query", query_params.q);
    properties.insert("Ages", ages);
    properties.insert("Language", language);
    analytics::event("Search", Some(properties));
}

pub fn search(state: &Rc<Home>) {
    state.loader.load(clone!(state => async move {
        search_async(state).await;
    }));
}
