use super::state::*;
use dominator::clone;
use shared::{
    api::endpoints::{session, user, ApiEndpoint},
    domain::{session::*, user::*},
    error::EmptyError,
};
use std::rc::Rc;
use utils::{prelude::*, storage};

pub fn signin_email(state: Rc<State>) {
    state.clear_email_status();
    state.clear_password_status();

    state.loader.load(clone!(state => async move {
        let email:String = state.email.borrow().clone();
        let password:String = state.password.borrow().clone();

        let (resp, _):(Result<CreateSessionResponse, EmptyError>, u16) = api_with_basic_token_status(session::Create::PATH, &email, &password, session::Create::METHOD, None::<()>).await;

        match resp {
            Ok(resp) => {
                match resp {
                    CreateSessionResponse::Login(resp) => {
                        do_success(&resp.csrf);
                    },
                    CreateSessionResponse::Register{response: _, oauth_profile: _} => {
                        panic!("didn't expect register response here!");
                    }
                }
            },
            Err(_err) => {
                state.status.set(Some(Status::BadCredentials));
            }
        }
    }));
}

pub fn signin_google(state: Rc<State>) {
    state.clear_email_status();
    state.clear_password_status();

    state.loader.load(async {
        crate::oauth::actions::redirect(GetOAuthUrlServiceKind::Google, OAuthUrlKind::Login).await;
    });
}

pub fn forgot_password(state: Rc<State>) {
    state.clear_password_status();

    state.loader.load(clone!(state => async move {
        let email:String = state.email.borrow().clone();

        let query = ResetPasswordRequest {
            email
        };

        let resp:Result<(), EmptyError> = api_no_auth_empty(user::ResetPassword::PATH, user::ResetPassword::METHOD, Some(query)).await;

        match resp {
            Ok(_) => {
                state.status.set(Some(Status::PasswordResetSent));
            },
            Err(_err) => {
                log::error!("Got error!")
            }
        }
    }));
}

pub fn go_register(_state: Rc<State>) {
    let route: String = Route::User(UserRoute::Register).into();
    dominator::routing::go_to_url(&route);
}

pub fn status_redirect(status: Option<Status>) {
    if let Some(status) = status {
        match status {
            Status::ConfirmEmail(email) => {
                let route: String = Route::User(UserRoute::SendEmailConfirmation(email)).into();
                dominator::routing::go_to_url(&route);
            }
            _ => {}
        }
    }
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
