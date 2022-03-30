use crate::unwrap::UnwrapJiExt;
use wasm_bindgen::prelude::*;
use web_sys::{window, Storage};

pub const CSRF_STORAGE_NAME: &str = "X-CSRF";

pub fn load_csrf_token() -> Option<String> {
    let res = get_local_storage()
        .unwrap_ji()
        .get(CSRF_STORAGE_NAME)
        .unwrap_ji();

    if res.is_none() {
        log::warn!("unable to load CSRF!");
    }

    res
}

pub fn save_csrf_token(csrf: &str) {
    let local_storage = get_local_storage().unwrap_ji();

    local_storage.set(CSRF_STORAGE_NAME, csrf).unwrap_ji()
}

pub fn delete_csrf_token() -> Result<(), JsValue> {
    let local_storage = get_local_storage().unwrap_ji();

    local_storage.remove_item(CSRF_STORAGE_NAME)
}

pub fn get_local_storage() -> Result<Storage, JsValue> {
    window()
        .unwrap_ji()
        .local_storage()?
        .ok_or_else(|| JsValue::from_str("could not get local storage!"))
}
