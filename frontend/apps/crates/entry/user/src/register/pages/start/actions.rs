use std::rc::Rc;
use super::state::*;
use shared::{
    api::endpoints::{ApiEndpoint, user::*, session::*},
    domain::session::*,
    error::EmptyError
};
use utils::{
    routes::*,
    firebase::*,
    fetch::{api_with_token, api_no_auth},
    storage,
};
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures::future::ready;
use crate::register::state::{Step, StartData};

pub fn register_email(state: Rc<State>) {
    state.clear_email_status();
    state.clear_password_status();

    let mut early_exit = false;
    if state.password_strength.get() != PasswordStrength::Strong {
        state.password_status.set(Some(PasswordStatus::PwWeak));
        early_exit = true;
    }


    let email:String = state.email.borrow().clone();
    let password:String = state.password.borrow().clone();

    if email.is_empty() {
        state.email_status.set(Some(EmailStatus::EmptyEmail));
        early_exit = true;
    }

    if early_exit {
        return;
    }

    state.loader.load(clone!(state => async move {

        /*
        let token_promise = unsafe { firebase_register_email(&email, &password) };

        match JsFuture::from(token_promise).await {
            Ok(info) => {
                let user:EmailUserInfo = serde_wasm_bindgen::from_value(info).unwrap_throw();
                next_step(state, user.token, user.email, user.email_verified);
            },

            Err(err) => { 
                match serde_wasm_bindgen::from_value::<FirebaseError>(err) {
                    Ok(err) => {
                        log::info!("{:?}", err);

                        match err.code.as_ref() {
                            "auth/invalid-email" => state.email_status.set(Some(EmailStatus::InvalidEmail)),
                            "auth/email-already-in-use" => state.email_status.set(Some(EmailStatus::EmailExists)),
                            //TODO - remove this check?
                            //If we trust our own vetting - should be fine
                            //and it's possible for the two approaches to be out of sync
                            "auth/weak-password" => {
                                log::warn!("firebase says it's weak... this shouldn't happen!");
                                state.password_strength.set(PasswordStrength::Weak);
                                state.password_status.set(Some(PasswordStatus::PwWeak))
                            },
                            _ => state.email_status.set(Some(EmailStatus::UnknownFirebase))
                        }
                    },
                    Err(_) => {
                        state.email_status.set(Some(EmailStatus::Technical));
                    }
                }
            }
        }
        */
    }));
}

pub fn register_google(state: Rc<State>) {
    state.clear_email_status();
    state.clear_password_status();
  

    state.loader.load(clone!(state => async move {

        let service_kind_str = serde_wasm_bindgen::to_value(&GetOAuthUrlServiceKind::Google)
            .unwrap_throw()
            .as_string()
            .unwrap_throw();

        let url_kind_str = serde_wasm_bindgen::to_value(&OAuthUrlKind::Register)
            .unwrap_throw()
            .as_string()
            .unwrap_throw();

        let path = GetOAuthUrl::PATH
            .replace("{service}", &service_kind_str)
            .replace("{kind}", &url_kind_str);
        if let Ok(resp) = api_no_auth::<GetOAuthUrlResponse, EmptyError, ()>(&path, GetOAuthUrl::METHOD, None).await {
            //web_sys::window().unwrap_throw().location().set_href(&resp.url);
            unsafe { crate::oauth_popup::actions::oauth_open_window(&resp.url, "oauth"); }
        }
    }));
}

pub fn handle_window_message(state: Rc<State>, evt: dominator_helpers::events::Message) {
    match evt.try_serde_data::<CreateSessionOAuthResponse>() {
        Ok(resp) => {
            match resp {
                CreateSessionOAuthResponse::Login {csrf} => {
                    log::info!("Login with {}", csrf);
                },
                CreateSessionOAuthResponse::CreateUser {csrf} => {
                    log::info!("Register with {}", csrf);
                },
                _ => {
                    log::info!("some other resp?");
                }
            }
        },
        Err(_) => {
            log::info!("couldn't deserialize window message into oauth");
        }
    }
}

fn next_step(state: Rc<State>, token: String, email: String, email_verified: bool) {
    state.step.set(Step::One(StartData{token, email, email_verified}));
}


pub fn update_password_strength(state: &Rc<State>) {
    let password:&str = &state.password.borrow();
    state.password_strength.set(PasswordStrength::Strong);
    //TODO...

}
