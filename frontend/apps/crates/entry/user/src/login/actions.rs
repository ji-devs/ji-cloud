use std::rc::Rc;
use super::state::*;
use shared::{
    api::endpoints::{ApiEndpoint, user, session},
    domain::{user::*, session::*},
    error::EmptyError
};
use utils::{
    routes::*,
    storage,
    prelude::*,
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
        let email:String = state.email.borrow().clone();
        let password:String = state.password.borrow().clone();

        let (resp, _):(Result<CreateSessionResponse, EmptyError>, u16) = api_with_basic_token_status(&session::Create::PATH, &email, &password, session::Create::METHOD, None::<()>).await;
       
        match resp {
            Ok(resp) => {
                match resp {
                    CreateSessionResponse::Login(resp) => {
                        do_success(&resp.csrf);
                    },
                    CreateSessionResponse::Register{response, oauth_profile} => {
                        panic!("didn't expect register response here!");
                    }
                }
            }, 
            Err(err) => {
                state.status.set(Some(Status::BadCredentials));
            }
        }
    }));
}

pub fn signin_google(state: Rc<State>) {
    state.clear_email_status();
    state.clear_password_status();

    state.loader.load(clone!(state => async move {
        crate::oauth::actions::redirect(GetOAuthUrlServiceKind::Google, OAuthUrlKind::Login).await;
    }));
}

pub fn forgot_password(state: Rc<State>) {
    state.clear_password_status();

    state.loader.load(clone!(state => async move {
        let email:String = state.email.borrow().clone();

        let query = ResetPasswordRequest {
            email
        };

        let resp:Result<(), EmptyError> = api_no_auth_empty(&user::ResetPassword::PATH, user::ResetPassword::METHOD, Some(query)).await;

        
        match resp {
            Ok(_) => {
                state.status.set(Some(Status::PasswordResetSent));
            }, 
            Err(err) => {
                log::error!("Got error!")
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
            Status::ConfirmEmail(email) => {
                let route:String = Route::User(UserRoute::SendEmailConfirmation(email)).into();
                dominator::routing::go_to_url(&route);
            },
            _ => {}
        }
    }
}
//// PRIVATE HELPERS /////


pub fn do_success(csrf:&str) {
    storage::save_csrf_token(&csrf);
    let route:String = Route::User(UserRoute::Profile(ProfileSection::Landing)).into();
    dominator::routing::go_to_url(&route);
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum SigninKind {
    Google,
    Email
}

