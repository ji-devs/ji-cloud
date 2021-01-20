use std::rc::Rc;
use super::state::*;
use shared::{
    api::endpoints::{ApiEndpoint, user::*,},
    domain::auth::SigninSuccess,
    error::EmptyError
};
use utils::{
    routes::*,
    fetch::api_with_token,
    storage,
};
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use crate::firebase::*;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures::future::ready;

pub fn signin_email(state: Rc<State>) {
    state.clear_email_status();
    state.clear_password_status();

    state.loader.load(clone!(state => async move {

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
    }));
}

pub fn signin_google(state: Rc<State>) {
    state.clear_email_status();
    state.clear_password_status();

    state.loader.load(clone!(state => async move {
        let token_promise = unsafe { firebase_signin_google() };
        match signin(token_promise, SigninKind::Google).await {
            Ok(csrf) => {
                do_success(&state, csrf);
            },
            Err(err) => {
                state.status.set(err);
            }
        }
    }));
}

pub fn forgot_password(state: Rc<State>) {
    state.clear_password_status();

    state.loader.load(clone!(state => async move {
        let email:String = state.email.borrow().clone();
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
            Status::NoSuchDbUser(user) => {
                let route:String = Route::User(UserRoute::ContinueRegistration(user)).into();
                dominator::routing::go_to_url(&route);
            },
            _ => {}
        }
    }
}
//// PRIVATE HELPERS /////
async fn signin(token_promise:js_sys::Promise, kind:SigninKind) -> Result<String, Option<Status>> {
    match signin_firebase(token_promise, kind).await {
        Ok(user) => {
            signin_api(user).await
        },
        Err(err) => Err(err)
    }
}


async fn signin_firebase(token_promise:js_sys::Promise, kind:SigninKind) -> Result<FirebaseUserInfo, Option<Status>> {
    match JsFuture::from(token_promise).await {
        Ok(user) => {
            Ok(
                match kind {
                    SigninKind::Email => FirebaseUserInfo::Email(serde_wasm_bindgen::from_value(user).unwrap_throw()),
                    SigninKind::Google => FirebaseUserInfo::Google(serde_wasm_bindgen::from_value(user).unwrap_throw()),
                }
            )
        },
        Err(err) => {
            let err_status = Status::from_firebase_err(err);
            //always return errors for email method
            //if google, only if the error is ConfirmEmail
            if kind == SigninKind::Email {
                Err(Some(err_status))
            } else {
                match err_status {
                    Status::ConfirmEmail => Err(Some(err_status)),
                    _ => Err(None)
                }
            }
        }
    }
}

async fn signin_api(user:FirebaseUserInfo) -> Result<String, Option<Status>> {

    let resp:Result<SigninSuccess, EmptyError> = 
        api_with_token::< _, _, ()>(&Signin::PATH, &user.token(), Signin::METHOD, None).await;
    
    match resp {
        Ok(data) => Ok(data.csrf), 
        Err(_) => Err(Some(Status::NoSuchDbUser(user)))
    }
}

fn do_success(state:&State, csrf:String) {
    storage::save_csrf_token(&csrf);

    let route:String = Route::User(UserRoute::Profile(ProfileSection::Landing)).into();
    dominator::routing::go_to_url(&route);

    ///generally speaking this kind of thing isn't necessary
    ///futures will just resolve and be dropped as part of the flow
    ///but because the oauth flow here opens a separate window
    ///it's more at risk to leave dangling Futures
    ///specifically, here, dangling futures which hold the Rc that holds it
    ///thereby creating a cycle, we need to break by cancelling that future
    ///see: https://github.com/jewish-interactive/ji-cloud/issues/78
    state.loader.cancel();
}

/// Data Helpers
pub type FirebaseId = String;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SigninKind {
    Google,
    Email
}

