use wasm_bindgen::prelude::*;
use js_sys::Promise;
use ji_cloud_shared::frontend::settings::Settings;

#[wasm_bindgen(module = "/js/firebase.js")]
extern "C" {
    pub fn init_firebase(dev:bool);
    pub fn get_firebase_signin_email(username:&str, password:&str) -> Promise;
    pub fn get_firebase_signin_google() -> Promise;
    pub fn get_firebase_register_google() -> Promise;
}


pub fn setup(settings:&Settings) {
    init_firebase(settings.firebase_dev);
}
