use wasm_bindgen::prelude::*;
use js_sys::Promise;
use core::settings::Settings;
use serde::{Serialize, Deserialize};

#[wasm_bindgen(module = "/js/firebase.js")]
extern "C" {
    pub fn init_firebase(dev:bool);
    pub fn firebase_signin_email(email:&str, password:&str) -> Promise;
    pub fn firebase_register_email(email:&str, password:&str) -> Promise;
    pub fn firebase_signin_google() -> Promise;
    pub fn firebase_register_google() -> Promise;
    pub fn firebase_forgot_password(email:&str) -> Promise;
}

#[derive(Deserialize, Debug)]
pub struct FirebaseError {
    pub code: String 
}

#[derive(Deserialize, Debug)]
pub struct GoogleRegisterInfo {
    pub avatar: String,
    pub email: String,
    pub name: String,
    pub token: String,
    pub firebase_id: String,
    pub email_verified: bool,
}

#[derive(Deserialize, Debug)]
pub struct EmailRegisterInfo {
    pub token: String,
    pub firebase_id: String,
    pub email_verified: bool,
}

pub fn setup(settings:&Settings) {
    unsafe { init_firebase(settings.firebase_dev); }
}
