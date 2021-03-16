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
    storage,
};
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures::future::ready;
use crate::register::state::{Step};
use zxcvbn::zxcvbn;

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
        crate::oauth::actions::redirect(GetOAuthUrlServiceKind::Google, OAuthUrlKind::Register).await;
    }));
}

pub fn update_password_strength(state: &Rc<State>) {
    let password:&str = &state.password.borrow();
    if password.is_empty() {
        state.password_strength.set(PasswordStrength::None);
    } else {
        let estimate = zxcvbn(password, &[]).unwrap_throw();
        state.password_strength.set(estimate.into());
    }

}
