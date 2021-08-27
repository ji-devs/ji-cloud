use std::rc::Rc;

use dominator::clone;
use shared::{api::{ApiEndpoint, endpoints::{self, user::Profile}}, domain::user::UserProfile, error::EmptyError};
use utils::{prelude::{ApiEndpointExt, api_with_auth_status}, storage::delete_csrf_token};

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

pub fn logout(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let local = delete_csrf_token();
        let server = endpoints::session::Delete::api_with_auth_empty(None).await;

        match (local, server) {
            (Ok(_), Ok(_)) => {
                state.logged_in.set(LoggedInState::LoggedOut);
            },
            _ => {
                let _ = web_sys::window().unwrap().alert_with_message("Error logging out!");
            },
        }
    }));
}
