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
use futures_signals::signal::{Mutable, SignalExt};
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

struct Foo {}
impl Foo {
    pub fn hello(&self) {
        log::info!("foo");
    }
}
impl Drop for Foo {
    fn drop(&mut self) {
        log::info!("DROPPED");
    }

}

//TODO - figure out how to make this spawn_local drop
//foo is just a placeholder to test (try removing .await to see it drop)
pub fn side_effects(page:Rc<SigninPage>) {
    let foo = Foo{};

    spawn_local(clone!(page => async move {
        foo.hello();
        page.status.signal_cloned().for_each(|status| {
            if let Some(status) = status {
                match status {
                    SigninStatus::Success(csrf) => {
                        storage::save_csrf_token(&csrf);
                        dominator::routing::go_to_url("/user/profile");
                        //dominator::routing::go_to_url( Route::User(UserRoute::Profile).into());
                        
                    },
                    _ => { 
                        log::info!("status: {:?}", status);
                    }
                }
            }

            ready(())
        }).await;
    }));
}

pub fn signin_google(page:Rc<SigninPage>) {
   
    page.status.set(Some(SigninStatus::Busy));

    let token_promise = unsafe { get_firebase_signin_google() };

    spawn_local(clone!(page => async move {
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


    }));
}

pub fn signin_email(status:&Mutable<Option<SigninStatus>>, email:&str, pw:&str) {
    status.set(Some(SigninStatus::Busy));

    log::info!("signin clicked! email: {} pw: {}", email, pw);
}
