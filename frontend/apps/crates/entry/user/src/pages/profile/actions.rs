use shared::{
    api::endpoints::{ApiEndpoint, user::*,},
    domain::user::UserProfile,
    error::{
        auth::RegisterError,
        user::NoSuchUserError
    }
};
use utils::{
    routes::{Route, UserRoute},
    fetch::api_with_auth,
    storage,
};
use serde::{Serialize, Deserialize};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use crate::firebase::firebase_register_google;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator::clone;
use std::rc::Rc;
use super::ProfilePage;
use futures::future::ready;

//temp
use futures::future::poll_fn;
use futures::task::{Context, Poll};

pub async fn load_profile(page:Rc<ProfilePage>) {
    let resp:Result<UserProfile, NoSuchUserError> = api_with_auth::< _, _, ()>(&Profile::PATH, Profile::METHOD, None).await;

    match resp {
        Ok(profile) => {
            page.status.set(Some(Ok(profile)));
        }, 
        Err(_) => {
            page.status.set(Some(Err(())))
        }
    }
}
