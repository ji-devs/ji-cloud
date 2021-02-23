use shared::{
    api::endpoints::{ApiEndpoint, user::*, session::*},
    domain::session::*,
    error::EmptyError
};
use utils::{
    routes::*,
    fetch::{api_with_token, api_no_auth},
    storage,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Window;

#[wasm_bindgen(module = "/js/oauth.js")]
extern "C" {
    pub fn oauth_open_window(url:&str, name:&str);
}


pub async fn finalize(req: CreateSessionOAuthRequest, redirect_kind: OAuthUrlKind) {
    
    if let Ok(resp) = api_no_auth::<CreateSessionOAuthResponse, EmptyError, _>(&CreateOAuth::PATH, CreateOAuth::METHOD, Some(req)).await {

        match resp {
            CreateSessionOAuthResponse::Login {csrf} => {
                storage::save_csrf_token(&csrf);
                let route:String = Route::User(UserRoute::Profile(ProfileSection::Landing)).into();
                dominator::routing::go_to_url(&route);
            },
            CreateSessionOAuthResponse::CreateUser {csrf} => {
                match redirect_kind {
                    OAuthUrlKind::Register => {
                        storage::save_csrf_token(&csrf);
                        let route:String = Route::User(UserRoute::ContinueRegistration).into();
                        dominator::routing::go_to_url(&route);
                    }
                    OAuthUrlKind::Login => {
                        web_sys::window().unwrap_throw().alert_with_message(crate::strings::STR_AUTH_OAUTH_LOGIN_FAIL);
                        let route:String = Route::User(UserRoute::Register).into();
                        dominator::routing::go_to_url(&route);
                    }
                }
            },
            _ => {}
        }
    }

}
