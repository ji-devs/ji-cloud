use shared::{
    api::endpoints::{ApiEndpoint, user::*,},
    domain::auth::SigninSuccess,
    error::user::NoSuchUserError,
};
use core::{
    routes::{Route, UserRoute},
    fetch::api_with_token,
    storage,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use crate::utils::firebase::*;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator::clone;
use std::rc::Rc;
use super::dom::{SigninPage, SigninInfo};
use futures::future::ready;
//temp
use futures::future::poll_fn;
use futures::task::{Context, Poll};
#[derive(Debug, Clone)]
pub enum SigninStatus {
    Busy,
    NoSuchUser,
    BadPassword,
    UnknownFirebase,
    Technical,
    PasswordResetSent,
    InvalidEmail,
    ConfirmEmail
}

impl SigninStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Busy => "logging in...",
            Self::NoSuchUser => "no such user!",
            Self::BadPassword => "wrong password!",
            Self::Technical => "technical error!",
            Self::UnknownFirebase => "firebase error!",
            Self::PasswordResetSent => "password reset link sent!",
            Self::InvalidEmail => "invalid email",
            Self::ConfirmEmail => "need to confirm your email!"
        }.to_string()
    }

    pub fn from_firebase_err(err:JsValue) -> Self {
        match serde_wasm_bindgen::from_value::<FirebaseError>(err) {
            Ok(err) => {
                let code:&str = err.code.as_ref();
                let status = match code {
                    "auth/wrong-password" => Self::BadPassword,
                    "auth/user-not-found" => Self::NoSuchUser,
                    "auth/invalid-email" => Self::InvalidEmail,
                    "internal/confirm-email" => Self::ConfirmEmail,
                    _ => {
                        log::warn!("firebase error: {}", code);
                        Self::UnknownFirebase
                    }
                };
                status
            },
            Err(_) => {
                Self::Technical
            }

        }
    }
}

pub fn do_success(page:&SigninPage, csrf:String) {
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

pub async fn signin_google() -> Result<String, Option<SigninStatus>> {
    let token_promise = unsafe { firebase_signin_google() };
    signin_firebase(token_promise, true).await
}

pub async fn signin_email(email:&str, password: &str) -> Result<String, Option<SigninStatus>> {
    let token_promise = unsafe { firebase_signin_email(&email, &password) };
    signin_firebase(token_promise, false).await
}

async fn signin_firebase(token_promise:js_sys::Promise, error_is_cancel:bool) -> Result<String, Option<SigninStatus>> {
    match JsFuture::from(token_promise).await {
        Ok(token) => {
            let token = token.as_string().unwrap_throw();
            let resp:Result<SigninSuccess, NoSuchUserError> = 
                api_with_token::< _, _, ()>(&Signin::PATH, &token, Signin::METHOD, None).await;
            
            match resp {
                Ok(data) => Ok(data.csrf), 
                Err(_) => Err(Some(SigninStatus::NoSuchUser))
            }
        },
        Err(err) => {
            if error_is_cancel {
                Err(None)
            } else {
                Err(Some(SigninStatus::from_firebase_err(err)))
            }
        }
    }
}


pub async fn forgot_password(email:&str) -> Result<(), SigninStatus> {
    let token_promise = unsafe { firebase_forgot_password(&email) };
    JsFuture::from(token_promise).await
        .map(|_| ())
        .map_err(|err| SigninStatus::from_firebase_err(err))
}
