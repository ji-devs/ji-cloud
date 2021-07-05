use web_sys::{window, Storage};
use wasm_bindgen::prelude::*;
use crate::unwrap::UnwrapJiExt;

pub const CSRF_STORAGE_NAME:&'static str = "X-CSRF";

pub fn load_csrf_token() -> Option<String> {
    get_local_storage()
        .unwrap_ji()
        .get(CSRF_STORAGE_NAME)
        .unwrap_ji()
}

pub fn save_csrf_token(csrf:&str) {
    let local_storage = get_local_storage().unwrap_ji();

    local_storage.set(CSRF_STORAGE_NAME, csrf).unwrap_ji()
}
pub fn get_local_storage() -> Result<Storage, JsValue> {
    window().unwrap_ji()
        .local_storage()?
        .ok_or(JsValue::from_str("could not get local storage!"))
}
