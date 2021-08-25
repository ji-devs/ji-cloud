use std::rc::Rc;

use dominator::clone;
use futures::future::join;
use shared::{
    api::endpoints::{ApiEndpoint, meta, user},
    domain::{meta::MetadataResponse, user::{PatchProfileRequest, UserProfile}},
    error::EmptyError
};

use utils::{fetch::api_with_auth, prelude::*, unwrap::UnwrapJiExt};
use super::state::State;

pub fn load_initial_data(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        join(
            load_profile(Rc::clone(&state)),
            load_metadata(Rc::clone(&state))
        ).await;
    }));
}

async fn load_profile(state: Rc<State>) {
    //let resp:Result<UserProfile, EmptyError> = api_with_auth::< _, _, ()>(&user::Profile::PATH, user::Profile::METHOD, None).await;
    let resp = user::Profile::api_with_auth(None).await;

    state.user.fill_from_user(resp.unwrap_ji());
}

async fn load_metadata(state: Rc<State>) {
    match api_with_auth::<MetadataResponse, EmptyError, ()>(meta::Get::PATH, meta::Get::METHOD, None).await {
        Err(_) => {},
        Ok(res) => {
            state.metadata.set(Some(res));
        },
    };
}

pub fn save_profile(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let info = state.user.to_update();

        //match api_with_auth_empty::<EmptyError, PatchProfileRequest>(user::PatchProfile::PATH, user::PatchProfile::METHOD, Some(info)).await {
        match user::PatchProfile::api_with_auth_empty(Some(info)).await {
            Err(_) => {},
            Ok(_) => {},
        };
    }));
}
