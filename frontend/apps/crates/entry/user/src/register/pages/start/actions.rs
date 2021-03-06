use std::rc::Rc;
use super::state::*;
use shared::{
    api::endpoints::{ApiEndpoint, user, session::*},
    domain::{user::CreateUserRequest, session::*},
    error::EmptyError
};
use utils::{
    routes::*,
    firebase::*,
    prelude::*,
    storage,
};
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures::future::ready;
use crate::{
    register::state::{Step},
    password::state::*
};
use zxcvbn::zxcvbn;

pub fn register_email(state: Rc<State>) {
    state.clear_email_status();
    state.password.clear_status();

    let mut early_exit = false;

    if state.password.strength.get() != PasswordStrength::Strong {
        state.password.status.set(Some(PasswordStatus::PwWeak));
        early_exit = true;
    }


    let email:String = state.email.borrow().clone();
    let password:String = state.password.value.borrow().clone();

    if email.is_empty() {
        state.email_status.set(Some(EmailStatus::EmptyEmail));
        early_exit = true;
    }

    if early_exit {
        return;
    }

    state.loader.load(clone!(state => async move {
        let query = CreateUserRequest {
            email: email.clone(),
            password
        };

        let (resp, status):(Result<(), EmptyError>, u16) = api_no_auth_empty_status(&user::Create::PATH, user::Create::METHOD, Some(query)).await;

        
        match resp {
            Ok(_) => {
                let route:String = Route::User(UserRoute::SendEmailConfirmation(email)).into();
                dominator::routing::go_to_url(&route);
            }, 
            Err(err) => {
                if status == 409 {
                    state.email_status.set(Some(EmailStatus::EmailExists));
                } else {
                    state.email_status.set(Some(EmailStatus::InvalidEmail));
                }
            }
        }
    }));
}

pub fn register_google(state: Rc<State>) {
    state.clear_email_status();
    state.password.clear_status();

    state.loader.load(clone!(state => async move {
        crate::oauth::actions::redirect(GetOAuthUrlServiceKind::Google, OAuthUrlKind::Register).await;
    }));
}

