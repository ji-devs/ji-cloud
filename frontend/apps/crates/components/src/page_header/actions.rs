use std::rc::Rc;

use dominator::clone;
use shared::{
    api::{endpoints::user::Profile, ApiEndpoint},
    domain::user::UserProfile,
    error::EmptyError,
};
use utils::prelude::api_with_auth_status;

use super::state::{LoggedInState, State};

pub fn fetch_profile(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let (result, status) = api_with_auth_status::<UserProfile, EmptyError, ()>(&Profile::PATH, Profile::METHOD, None).await;

        match status  {
            401 | 403 => {
                state.logged_in.set(LoggedInState::LoggedOut)
            }
            _ => {
                match result {
                    Err(_) => {
                        log::info!("error fetching profile");
                    },
                    Ok(profile) => {
                        state.logged_in.set(LoggedInState::LoggedIn(profile))
                    }
                }
            }
        };
    }));
}

pub fn logout(_state: Rc<State>) {
    // state.loader.load(clone!(state => async move {
        
    // }));
    let _ = web_sys::window().unwrap().alert_with_message("not implemented");
}
