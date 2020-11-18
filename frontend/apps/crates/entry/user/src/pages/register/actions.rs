use shared::{
    api::endpoints::{ApiEndpoint, user::*,},
    domain::auth::{RegisterRequest, RegisterSuccess},
    error::{
        auth::RegisterError,
        user::NoSuchUserError
    }
};
use utils::{
    path::api_url,
    routes::{Route, UserRoute},
    fetch::{FetchResult, api_with_token},
};
use serde::{Serialize, Deserialize};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use crate::firebase::*;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator::clone;
use std::rc::Rc;
use super::dom::*;
use futures::future::ready;

//temp
use futures::future::poll_fn;
use futures::task::{Context, Poll};


pub async fn register_email(email: &str, pw: &str) -> Result<String, RegisterStatus> {
    let token_promise = unsafe { firebase_register_email(email, pw) };

    JsFuture::from(token_promise).await
        .map(|info| {
            let user:EmailRegisterInfo = serde_wasm_bindgen::from_value(info).unwrap_throw();
            user.token
        })
        .map_err(|err| {
            match serde_wasm_bindgen::from_value::<FirebaseError>(err) {
                Ok(err) => {
                    match err.code.as_ref() {
                        "auth/email-already-in-use" => RegisterStatus::EmailExists,
                        "auth/weak-password" => RegisterStatus::PwWeak,
                        _ => RegisterStatus::UnknownFirebase
                    }
                },
                Err(uhh) => {
                    RegisterStatus::Technical
                }
            }
        })
}

pub async fn register_google() -> Result<GoogleRegisterInfo, Option<RegisterStatus>> {
    let token_promise = unsafe { firebase_register_google() };

    JsFuture::from(token_promise).await
        .map(|info| {
            serde_wasm_bindgen::from_value::<GoogleRegisterInfo>(info).unwrap_throw()
        })
        .map_err(|err| {
            None
            /*
            match serde_wasm_bindgen::from_value::<FirebaseError>(err) {
                Ok(err) => {
                    match err.code.as_ref() {
                        "auth/email-already-in-use" => RegisterStatus::EmailExists,
                        "auth/weak-password" => RegisterStatus::PwWeak,
                        _ => RegisterStatus::UnknownFirebase
                    }
                },
                Err(uhh) => {
                    RegisterStatus::Technical
                }
            }
            */
        })
}
pub async fn create_user(
    token: String,
    username: String,
    given_name: String,
    family_name: String,
    email: String,
) -> Result<String, RegisterStatus> {
    let req = RegisterRequest {
        username,
        email,
        given_name,
        family_name,
        over_18: true,
        language: "en".to_string(),
        locale: "en".to_string(),
        timezone: chrono_tz::Tz::Asia__Jerusalem,
        opt_into_edu_resources: true,
        organization: "ji".to_string()
    };

    let resp:FetchResult<RegisterSuccess, RegisterError> = api_with_token(&api_url(Register::PATH), &token, Register::METHOD, Some(req)).await;

    match resp {
        Ok(resp) => match resp {
            RegisterSuccess::Signin(csrf) => Ok(csrf),
            RegisterSuccess::ConfirmEmail => Err(RegisterStatus::ConfirmEmail)
        }, 
        Err(_) => {
            Err(RegisterStatus::Technical)
        }
    }
}

/*
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
