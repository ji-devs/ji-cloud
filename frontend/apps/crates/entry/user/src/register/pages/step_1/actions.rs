use super::state::*;
use std::rc::Rc;
use shared::{
    api::endpoints::{ApiEndpoint, user::*,},
    domain::user::*,
    error::EmptyError
};
use utils::{
    routes::*,
    firebase::*,
    fetch::api_no_auth,
    storage,
};
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use crate::register::state::{Step, Step1Data};

pub fn submit(state: Rc<State>) {
    let mut ready = true;

    if !*state.over_18.borrow() {
        state.over_18_status.set(Some(Over18Error::Unchecked));
        ready = false;
    } 

    if state.firstname.borrow().is_empty() {
        state.firstname_status.set(Some(NameError::Empty));
        ready = false;
    }

    if state.lastname.borrow().is_empty() {
        state.lastname_status.set(Some(NameError::Empty));
        ready = false;
    }

    if state.username.borrow().is_empty() {
        state.username_status.set(Some(NameError::Empty));
        ready = false;
    }

    if !ready {
        return;
    }

    state.username_taken_loader.load(clone!(state => async move {
        if username_exists(state.username.borrow().clone()).await {
            state.username_status.set(Some(NameError::Exists));
        } else {
            next_step(state);
        }
    }));
}

fn next_step(state: Rc<State>) {
    state.step.set(Step::Two(Step1Data{
        firstname: state.firstname.borrow().clone(),
        lastname: state.lastname.borrow().clone(),
        username: state.username.borrow().clone(),
    }));
}
async fn username_exists(name:String) -> bool {

    let query = UserLookupQuery {
        id: None,
        name: Some(name) 
    };

    let resp:Result<OtherUser, EmptyError> = api_no_auth(&UserLookup::PATH, UserLookup::METHOD, Some(query)).await;

    resp.is_ok()
}
