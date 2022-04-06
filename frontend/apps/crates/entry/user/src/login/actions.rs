use super::state::*;
use dominator::clone;
use shared::{
    api::endpoints::{session, ApiEndpoint},
    domain::session::*,
    error::EmptyError,
};
use std::rc::Rc;
use utils::{prelude::*, storage};

const STR_INVALID_COMBINATION: &str = "Invalid email or password combination";

pub fn signin_email(state: Rc<LoginPage>) {
    state.tried_to_submit.set(true);

    if !state.email.email_acceptable() {
        return;
    }

    state.loader.load(clone!(state => async move {
        let email = state.email.get_value();
        let password = state.password.borrow().clone();

        let (resp, _):(Result<CreateSessionResponse, EmptyError>, u16) = api_with_basic_token_status(session::Create::PATH, &email, &password, session::Create::METHOD, None::<()>).await;

        match resp {
            Ok(resp) => {
                match resp {
                    CreateSessionResponse::Login(resp) => {
                        do_success(&resp.csrf);
                    },
                    CreateSessionResponse::Register{response, oauth_profile} => {
                        storage::save_csrf_token(&response.csrf);
                        let route = Route::User(UserRoute::ContinueRegistration(oauth_profile)).to_string();
                        dominator::routing::go_to_url(&route);
                    }
                }
            },
            Err(_err) => {
                state.email.set_error(STR_INVALID_COMBINATION);
                state.password_error.set(Some(STR_INVALID_COMBINATION));
            }
        }
    }));
}

pub fn signin_google(state: Rc<LoginPage>) {
    state.loader.load(async {
        crate::oauth::actions::redirect(GetOAuthUrlServiceKind::Google, OAuthUrlKind::Login).await;
    });
}

pub fn go_register(_state: Rc<LoginPage>) {
    let route: String = Route::User(UserRoute::Register(Default::default())).into();
    dominator::routing::go_to_url(&route);
}

//// PRIVATE HELPERS /////

pub fn do_success(csrf: &str) {
    storage::save_csrf_token(csrf);

    let location = web_sys::window().unwrap_ji().location();
    let origin = location.origin().unwrap_ji();
    let search_params = location.search().unwrap_ji();
    let search_params = web_sys::UrlSearchParams::new_with_str(&search_params).unwrap_ji();

    let url = search_params.get("redirect").unwrap_or_default();
    let url: String = js_sys::decode_uri_component(&url).unwrap_ji().into();

    let url = format!("{}{}", origin, url);

    let _ = location.set_href(&url);
}
