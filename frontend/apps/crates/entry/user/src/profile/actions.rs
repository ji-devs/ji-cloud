use shared::{
    api::endpoints::{ApiEndpoint, user::*,},
    domain::user::UserProfile,
    error::{
        auth::RegisterError,
        EmptyError,
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
use futures::future::ready;
use super::state::State;

pub async fn load_profile(state:Rc<State>) {
    let resp:Result<UserProfile, EmptyError> = api_with_auth::< _, _, ()>(&Profile::PATH, Profile::METHOD, None).await;

    match resp {
        Ok(profile) => {
            state.status.set(Some(Ok(profile)));
        }, 
        Err(_) => {
            state.status.set(Some(Err(())))
        }
    }
}
