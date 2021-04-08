use shared::{
    api::endpoints::{ApiEndpoint, user::*, session::*},
    domain::session::*,
    error::EmptyError
};
use utils::{
    routes::*,
    fetch::{api_no_auth, api_no_auth_with_credentials},
    storage,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Window;

#[wasm_bindgen(module = "/js/oauth.js")]
extern "C" {
    pub fn oauth_open_window(url:&str, name:&str);
}


pub async fn redirect(service_kind: GetOAuthUrlServiceKind, url_kind: OAuthUrlKind) {
    let service_kind_str = serde_wasm_bindgen::to_value(&service_kind)
        .unwrap_throw()
        .as_string()
        .unwrap_throw();

    let url_kind_str = serde_wasm_bindgen::to_value(&url_kind)
        .unwrap_throw()
        .as_string()
        .unwrap_throw();

    let path = GetOAuthUrl::PATH
        .replace("{service}", &service_kind_str)
        .replace("{kind}", &url_kind_str);
    if let Ok(resp) = api_no_auth::<GetOAuthUrlResponse, EmptyError, ()>(&path, GetOAuthUrl::METHOD, None).await {
        web_sys::window().unwrap_throw().location().set_href(&resp.url);
        //unsafe { crate::oauth::actions::oauth_open_window(&resp.url, "oauth"); }
    }
}
pub async fn finalize(req: CreateSessionOAuthRequest, url_kind: OAuthUrlKind) {
    
    if let Ok(resp) = api_no_auth_with_credentials::<CreateSessionResponse, EmptyError, _>(&CreateOAuth::PATH, CreateOAuth::METHOD, Some(req)).await {

        match resp {
            CreateSessionResponse::Login(resp) => {
                let csrf = resp.csrf;

                storage::save_csrf_token(&csrf);
                let route:String = Route::User(UserRoute::Profile(ProfileSection::Landing)).into();
                dominator::routing::go_to_url(&route);
            },
            CreateSessionResponse::Register(resp) => {
                let csrf = resp.csrf;

                match url_kind {
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
