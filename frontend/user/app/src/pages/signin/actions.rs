use shared::domain::{
    auth::SigninSuccess,
    user::NoSuchUserError
};
use core::{
    routes::{Route, UserRoute},
    fetch::{
        FetchResult,
        user::fetch_signin,
    },
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

//temp
use futures::future::poll_fn;
use futures::task::{Context, Poll};
#[derive(Debug, Clone)]
pub enum SigninStatus {
    Busy,
    NoSuchUser,
}

impl SigninStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Busy => "logging in...".to_string(),
            Self::NoSuchUser => "unable to log in!".to_string(),
        }
    }
}

fn do_success(page:&SigninPage, csrf:String) {
    storage::save_csrf_token(&csrf);

    let route:String = Route::User(UserRoute::Profile).into();
    dominator::routing::go_to_url(&route);

    ///generally speaking this kind of thing isn't necessary
    ///futures will just resolve and be dropped as part of the flow
    ///but because the oauth flow here opens a separate window
    ///it's more at risk to leave dangling Futures
    ///specifically, here, dangling futures which hold the Rc that holds it
    ///thereby creating a cycle, we need to break by cancelling that future
    ///see: https://github.com/jewish-interactive/ji-cloud/issues/78
    page.loader.cancel();
}

pub async fn signin_google(page:Rc<SigninPage>) {


    let token_promise = unsafe { get_firebase_signin_google() };

    match JsFuture::from(token_promise).await {
        Ok(token) => {
            let token = token.as_string().unwrap_throw();
            let resp:FetchResult<SigninSuccess, NoSuchUserError> = fetch_signin(&token).await;
            match resp {
                Ok(data) => do_success(&page, data.csrf),
                Err(_) => {
                    page.status.set(Some(SigninStatus::NoSuchUser))
                }
            }
        },
        Err(_) => {
            //not really an error, probably a cancel
            page.status.set(None);
        }
    };
}
