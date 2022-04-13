use dominator::clone;
use shared::domain::image::ImageSearchQuery;
use std::rc::Rc;
use utils::{fetch::api_with_auth, routes::*};

use super::state::*;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::image::*,
    error::EmptyError,
};

pub fn search(state: Rc<State>, query: ImageSearchQuery) {
    state.loader.load(clone!(state => async move {
        //update the address bar
        let route = Route::Admin(AdminRoute::ImageSearch(Some(query.clone())));
        route.push_state();

        //search
        match api_with_auth::<ImageSearchResponse, EmptyError, _>(endpoints::image::Search::PATH, endpoints::image::Search::METHOD, Some(query)).await {
            Ok(res) => {
                state.response.set(Some(res))
            },
            Err(_) => {
                todo!();
            },
        }
    }));
}
