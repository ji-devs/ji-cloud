use dominator::clone;
use shared::domain::image::ImageSearchQuery;
use std::rc::Rc;
use utils::{prelude::ApiEndpointExt, routes::*};

use super::state::*;
use shared::{api::endpoints, domain::image::*};

pub fn search(state: Rc<State>, query: ImageSearchQuery) {
    state.loader.load(clone!(state => async move {
        //update the address bar
        let route = Route::Admin(AdminRoute::ImageSearch(Some(query.clone())));
        route.push_state();

        //search
        match endpoints::image::Search::api_with_auth(ImageSearchPath(), Some(query)).await {
            Ok(res) => {
                state.response.set(Some(res))
            },
            Err(_) => {
                todo!();
            },
        }
    }));
}
