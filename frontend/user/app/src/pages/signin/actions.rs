use shared::{
    auth::SigninSuccess,
    user::NoSuchUserError
};
use core::{
    routes::{Route, UserRoute},
    fetch::user::fetch_signin,
    storage,
};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use crate::utils::firebase::get_firebase_signin_google;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator::clone;
use std::rc::Rc;
use super::SigninPage;
use futures::future::ready;

#[derive(Debug, Clone)]
pub enum SigninStatus {
    Success(String),
    Busy,
    NoSuchUser,
}

pub async fn run_side_effects(status: impl Signal<Item = Option<SigninStatus>>) {
    status.for_each(|status| {
        if let Some(status) = status {
            match status {
                SigninStatus::Success(csrf) => {
                    storage::save_csrf_token(&csrf);
                    dominator::routing::go_to_url( Route::User(UserRoute::Profile).into());
                    
                },
                _ => { 
                    log::info!("status: {:?}", status);
                }
            }
        }

        ready(())
    }).await;
}

pub async fn signin_google(page:Rc<SigninPage>) {


    let token_promise = unsafe { get_firebase_signin_google() };

    match JsFuture::from(token_promise).await {
        Ok(token) => {
            let token = token.as_string().unwrap_throw();
            let resp:Result<SigninSuccess, NoSuchUserError> = fetch_signin(&token).await;
            match resp {
                Ok(data) => page.status.set(Some(SigninStatus::Success(data.csrf))),
                Err(_) => page.status.set(Some(SigninStatus::NoSuchUser))
            }
        },
        Err(_) => {
            page.status.set(None);
        }
    };
}

pub async fn signin_email(page:Rc<SigninPage>) {

    let refs = page.refs.borrow();
    let refs = refs.as_ref().unwrap_throw();
    let email = refs.get_email();
    let pw = refs.get_pw();
    log::info!("signin clicked! email: {} pw: {}", email, pw);
}
