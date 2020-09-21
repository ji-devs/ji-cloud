use shared::{
    domain::auth::{RegisterRequest, RegisterSuccess},
    error::{
        auth::RegisterError,
        user::NoSuchUserError
    }
};
use core::{
    routes::{Route, UserRoute},
    fetch::{
        FetchResult,
        user::fetch_register,
    },
    storage,
};
use serde::{Serialize, Deserialize};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use crate::utils::firebase::get_firebase_register_google;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator::clone;
use std::rc::Rc;
use super::dom::*;
use futures::future::ready;

//temp
use futures::future::poll_fn;
use futures::task::{Context, Poll};
#[derive(Debug, Clone)]
pub enum RegisterStatus {
    Busy,
    Failure,
    ConfirmEmail,
}

impl RegisterStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Busy => "registering...".to_string(),
            Self::Failure => "failed to register!".to_string(),
            Self::ConfirmEmail => "confirm your email!".to_string(),

        }
    }
}
/*

#[derive(Deserialize)]
struct GoogleRegisterInfo {
    avatar: String,
    email: String,
    name: String,
    token: String
}

pub async fn register_google(page:Rc<RegisterPage>) {
    let token_promise = unsafe { get_firebase_register_google() };

    let refs = page.refs.borrow();
    let refs = refs.as_ref().unwrap_throw();

    let username = refs.username();
    let family_name = refs.family_name();
    let given_name = refs.given_name();
    match JsFuture::from(token_promise).await {
        Ok(info) => {
            let user:GoogleRegisterInfo = serde_wasm_bindgen::from_value(info).unwrap_throw();
            //let (first_name, last_name) = parse_name(&user.name);
            let req = RegisterRequest {
                username,
                email: user.email,
                over_18: true,
                given_name,
                family_name,
                language: "en".to_string(),
                locale: "en".to_string(),
                timezone: chrono_tz::Tz::Asia__Jerusalem,
                opt_into_edu_resources: true,
                organization: "ji".to_string()
            };

            let resp:FetchResult<RegisterSuccess, RegisterError> = fetch_register(&user.token, &req).await;
            match resp {
                Ok(resp) => match resp {
                    RegisterSuccess::Signin(csrf) => do_success(&page, csrf),
                    RegisterSuccess::ConfirmEmail => page.status.set(Some(RegisterStatus::ConfirmEmail)),
                }, 
                Err(_) => {
                    page.status.set(Some(RegisterStatus::Failure))
                }
            }
        },
        Err(_) => {
            //not really an error, probably a cancel
            page.status.set(None);
        }
    };
}
fn do_success(page:&RegisterPage, csrf:String) {
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

fn parse_name(name:&str) -> (String, String) {
    let names_split:Vec<&str> = 
        name
            .split_whitespace()
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
            .collect();
    
    let len = names_split.len();
    
    if len == 0 {
        ("".to_string(), "".to_string())
    } else if len == 1 {
        (names_split[0].to_string(), "".to_string())
    } else if len == 2 {
        (names_split[0].to_string(), names_split[1].to_string())
    } else {
        (names_split[0..len-1].join(" "), names_split[len-1].to_string())
    }
}
*/
