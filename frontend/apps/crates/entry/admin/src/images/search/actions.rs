use dominator::clone;
use shared::domain::image::{ImageId, ImageSearchQuery};
use std::rc::Rc;
use utils::{routes::*, fetch::api_with_auth};
use wasm_bindgen::prelude::*;
use super::state::*;
use shared::{
    api::{ApiEndpoint, endpoints},
    domain::{Publish, image::*, meta::*},
    error::{EmptyError, MetadataNotFound},
};

pub fn search(state: Rc<State>, query: ImageSearchQuery) {
    state.loader.load(clone!(state => async move {
        //update the address bar
        let route = Route::Admin(AdminRoute::ImageSearch(Some(query.clone())));
        route.push_state();

        //search
        match api_with_auth::<ImageSearchResponse, EmptyError, _>(&endpoints::image::Search::PATH, endpoints::image::Search::METHOD, Some(query)).await {
            Ok(res) => {
                state.response.set(Some(res))
            },
            Err(_) => {}
        }
    }));
}
