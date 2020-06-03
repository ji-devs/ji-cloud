use serde::{Serialize, Deserialize};
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use ji_cloud_shared::{
    user::UserRole,
    auth::SigninSuccess,
    response::ResultResponse,
    frontend::{
        fetch, 
        storage,
        routes::Route
    }
};
use js_sys::Promise;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use crate::{
    utils::firebase::get_firebase_signin_google,
    path::api_url
};

pub fn on_signin_success(csrf:&str) {
    storage::save_csrf_token(csrf);
    Route::Profile.redirect();
}

pub fn sign_in_email(username:&str, password:&str) {
    //_sign_in_email(username, password, format!("{}/user/login", SETTINGS.api_url_base));
}

pub fn signin_google<Happy: FnOnce(SigninSuccess) + 'static, Sad: FnOnce() + 'static>(on_happy:Happy, on_sad:Sad) {
    signin(get_firebase_signin_google(), on_happy, on_sad);
}

fn signin<Happy: FnOnce(SigninSuccess) + 'static, Sad: FnOnce() + 'static>(token_promise:Promise, on_happy: Happy, on_sad: Sad) {
    spawn_local(async {
        match JsFuture::from(token_promise).await {
            Ok(token) => {
                let token = token.as_string().unwrap_throw();
                match fetch::unwrapped::api_with_token_no_data::<SigninSuccess, ()>(&api_url("user/signin"), &token).await {
                    Ok(status) => on_happy(status), 
                    Err(err) => on_sad() 
                }
            },
            Err(_) => log::info!("error logging in!")
        }
    });
}
