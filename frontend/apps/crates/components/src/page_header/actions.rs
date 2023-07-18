use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::session::DeleteSessionPath};
use utils::{
    prelude::{get_user_cloned, ApiEndpointExt},
    routes::{LoginQuery, Route, UserRoute},
    storage::delete_csrf_token,
    unwrap::UnwrapJiExt,
};

use super::state::{LoggedInState, PageHeader};

pub fn fetch_profile(state: Rc<PageHeader>) {
    match get_user_cloned() {
        // Some(profile) => state.logged_in.set(LoggedInState::LoggedIn(&profile)),
        Some(profile) => state.logged_in.set(LoggedInState::LoggedIn(profile)),
        None => state.logged_in.set(LoggedInState::LoggedOut),
    }
}

pub fn logout(state: Rc<PageHeader>) {
    state.loader.load(clone!(state => async move {
        let server = endpoints::session::Delete::api_with_auth(DeleteSessionPath(), None).await;
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
