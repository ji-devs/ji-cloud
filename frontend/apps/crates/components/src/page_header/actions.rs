use std::rc::Rc;

use shared::{api::endpoints, domain::session::DeleteSessionPath};
use utils::window::navigate_to_login;
use utils::{
    prelude::{get_user_cloned, ApiEndpointExt},
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
    state.loader.load(async {
        let server = endpoints::session::Delete::api_with_auth(DeleteSessionPath(), None).await;
        let local = delete_csrf_token();

        match (local, server) {
            (Ok(_), Ok(_)) => {
                navigate_to_login();
            }
            _ => {
                let _ = web_sys::window()
                    .unwrap_ji()
                    .alert_with_message("Error logging out!");
            }
        }
    });
}
