use serde::{Serialize, Deserialize};
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use ji_cloud_shared::{
    user::UserRole,
    auth::{RegisterRequest, RegisterSuccess, RegisterError},
    response::ResultResponse,
    frontend::fetch
};
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use crate::{
    utils::{
        firebase::{get_firebase_register_google},
    },
    path::api_url
};

#[derive(Deserialize)]
struct GoogleRegisterInfo {
    avatar: String,
    email: String,
    name: String,
    token: String
}

pub fn register_google<Happy: FnOnce(RegisterSuccess) + 'static, Sad: FnOnce(RegisterError) + 'static>(on_happy:Happy, on_sad:Sad) {

    spawn_local(async {
        match JsFuture::from(get_firebase_register_google()).await {
            Ok(info) => {
                let user:GoogleRegisterInfo = serde_wasm_bindgen::from_value(info).unwrap_throw();
                let (first_name, last_name) = parse_name(&user.name);
                let req = RegisterRequest {
                    display_name: user.name,
                    first_name,
                    last_name,
                    email: user.email
                };
                match fetch::unwrapped::api_with_token::<RegisterSuccess, RegisterError, _>(&api_url("user/register"), &user.token, Some(req)).await {
                    Ok(resp) => on_happy(resp),
                    Err(err) => {
                        log::info!("Hmmm got error...");
                        on_sad(err) 
                    }
                }
            },
            Err(_) => log::info!("error logging in!")
        }
    });
}

fn parse_name(name:&str) -> (String, String) {
    let names_split:Vec<&str> = 
        name
            .split_whitespace()
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
            .collect();
    
    let len = names_split.len();
    
    if len == 0 {
        ("".to_string(), "".to_string())
    } else if len == 1 {
        (names_split[0].to_string(), "".to_string())
    } else if len == 2 {
        (names_split[0].to_string(), names_split[1].to_string())
    } else {
        (names_split[0..len-1].join(" "), names_split[len-1].to_string())
    }
}
