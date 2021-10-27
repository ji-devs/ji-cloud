use std::rc::Rc;

use dominator::clone;
use shared::{
    api::{
        endpoints::{self, user::Profile},
        ApiEndpoint,
    },
    domain::user::UserProfile,
    error::EmptyError,
};
use utils::{
    prelude::{api_with_auth_status, ApiEndpointExt},
    routes::{Route, UserRoute},
    storage::delete_csrf_token,
    unwrap::UnwrapJiExt,
};

use super::state::{LoggedInState, State};

pub fn fetch_profile(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let (result, status) = api_with_auth_status::<UserProfile, EmptyError, ()>(Profile::PATH, Profile::METHOD, None).await;

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
        let server = endpoints::session::Delete::api_with_auth_empty(None).await;
        let local = delete_csrf_token();

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

pub fn navigate_to_login() {
    let location = web_sys::window().unwrap_ji().location();
    let origin = location.origin().unwrap_ji();

    let redirect = format!(
        "{}{}",
        location.pathname().unwrap_ji(),
        location.search().unwrap_ji()
    );
    let redirect: String = js_sys::encode_uri_component(&redirect).into();

    let route: String = Route::User(UserRoute::Login(redirect)).to_string();

    let url = format!("{}{}", origin, route);

    let _ = location.set_href(&url);
}
