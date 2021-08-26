use once_cell::sync::OnceCell;
use std::fmt;
use cfg_if::cfg_if;
use shared::config::RemoteTarget;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use shared::domain::auth::AUTH_COOKIE_NAME;
use crate::unwrap::UnwrapJiExt;

pub mod settings;
pub mod user;

pub async fn init() {
    settings::init().await;
    user::init().await;
}
