use std::rc::Rc;
use super::state::*;
use shared::{
    api::endpoints::{ApiEndpoint, user::*, session::*},
    domain::session::*,
    error::EmptyError
};
use utils::{
    routes::*,
    fetch::{api_no_auth, api_with_token},
    storage,
};
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures::future::ready;

pub fn signin_email(state: Rc<State>) {
    state.clear_email_status();
    state.clear_password_status();

    state.loader.load(clone!(state => async move {
        /*
        let email:String = state.email.borrow().clone();
        let password:String = state.password.borrow().clone();

        let token_promise = unsafe { firebase_signin_email(&email, &password) };

        match signin(token_promise, SigninKind::Email).await {
            Ok(csrf) => {
                do_success(&state, csrf);
            },
            Err(err) => {
                state.status.set(err);
            }
        }
        */
    }));
}

pub fn signin_google(state: Rc<State>) {
    state.clear_email_status();
    state.clear_password_status();

    state.loader.load(clone!(state => async move {
        let service_kind_str = serde_wasm_bindgen::to_value(&GetOAuthUrlServiceKind::Google)
            .unwrap_throw()
            .as_string()
            .unwrap_throw();

        let url_kind_str = serde_wasm_bindgen::to_value(&OAuthUrlKind::Login)
            .unwrap_throw()
            .as_string()
            .unwrap_throw();

        let path = GetOAuthUrl::PATH
            .replace("{service}", &service_kind_str)
            .replace("{kind}", &url_kind_str);
        if let Ok(resp) = api_no_auth::<GetOAuthUrlResponse, EmptyError, ()>(&path, GetOAuthUrl::METHOD, None).await {
            web_sys::window().unwrap_throw().location().set_href(&resp.url);
            //unsafe { crate::oauth::actions::oauth_open_window(&resp.url, "oauth"); }
        }
    }));
}

pub fn forgot_password(state: Rc<State>) {
    state.clear_password_status();

    state.loader.load(clone!(state => async move {
        let email:String = state.email.borrow().clone();
        /*
        let token_promise = unsafe { firebase_forgot_password(&email) };
        let res = JsFuture::from(token_promise).await
            .map(|_| ())
            .map_err(|err| Status::from_firebase_err(err));

        match res {
            Ok(csrf) => {
                state.status.set(Some(Status::PasswordResetSent));
            },
            Err(err) => {
                state.status.set(Some(err));
            }
        }
        */
    }));
}

pub fn go_register(state: Rc<State>) {
    let route:String = Route::User(UserRoute::Register).into();
    dominator::routing::go_to_url(&route);
}

pub fn status_redirect(status:Option<Status>) {
    if let Some(status) = status {
        match status {
            Status::ConfirmEmail => {
                let route:String = Route::User(UserRoute::SendEmailConfirmation).into();
                dominator::routing::go_to_url(&route);
            },
            _ => {}
        }
    }
}
//// PRIVATE HELPERS /////


fn do_success(state:&State, csrf:String) {
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum SigninKind {
    Google,
    Email
}

