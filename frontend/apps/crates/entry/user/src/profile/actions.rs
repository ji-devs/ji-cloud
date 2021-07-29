use std::rc::Rc;

use dominator::clone;
use futures::future::join;
use shared::{
    api::endpoints::{
        ApiEndpoint,
        user::*,
        meta
    },
    domain::{
        meta::MetadataResponse,
        user::UserProfile
    },
    error::EmptyError
};

use utils::{fetch::api_with_auth, unwrap::UnwrapJiExt};
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
    let resp:Result<UserProfile, EmptyError> = api_with_auth::< _, _, ()>(&Profile::PATH, Profile::METHOD, None).await;

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
        log::info!("{}", info);
    }));
}
