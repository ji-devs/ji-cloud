use std::rc::Rc;

use dominator::clone;
use shared::api::endpoints;
use utils::{
    prelude::{get_user, ApiEndpointExt},
    routes::{LoginQuery, Route, UserRoute},
    storage::delete_csrf_token,
    unwrap::UnwrapJiExt,
};

use super::state::{LoggedInState, State};

pub fn fetch_profile(state: Rc<State>) {
    match get_user() {
        // Some(profile) => state.logged_in.set(LoggedInState::LoggedIn(&profile)),
        Some(profile) => state.logged_in.set(LoggedInState::LoggedIn(profile)),
        None => state.logged_in.set(LoggedInState::LoggedOut),
    }
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
                let _ = web_sys::window().unwrap_ji().alert_with_message("Error logging out!");
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

    let route: String = Route::User(UserRoute::Login(LoginQuery::redirect(redirect))).to_string();

    let url = format!("{}{}", origin, route);

    let _ = location.set_href(&url);
}
