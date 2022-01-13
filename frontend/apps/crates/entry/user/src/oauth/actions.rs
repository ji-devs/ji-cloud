use shared::{
    api::endpoints::{session::*, ApiEndpoint},
    domain::session::*,
    error::EmptyError,
};
use utils::{
    fetch::{api_no_auth, api_no_auth_with_credentials},
    routes::*,
    storage, unwrap::UnwrapJiExt,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/oauth.js")]
extern "C" {
    pub fn oauth_open_window(url: &str, name: &str);
}

pub async fn redirect(service_kind: GetOAuthUrlServiceKind, url_kind: OAuthUrlKind) {
    let service_kind_str = serde_wasm_bindgen::to_value(&service_kind)
        .unwrap_ji()
        .as_string()
        .unwrap_ji();

    let url_kind_str = serde_wasm_bindgen::to_value(&url_kind)
        .unwrap_ji()
        .as_string()
        .unwrap_ji();

    let path = GetOAuthUrl::PATH
        .replace("{service}", &service_kind_str)
        .replace("{kind}", &url_kind_str);
    if let Ok(resp) =
        api_no_auth::<GetOAuthUrlResponse, EmptyError, ()>(&path, GetOAuthUrl::METHOD, None).await
    {
        let _ = web_sys::window()
            .unwrap_ji()
            .location()
            .set_href(&resp.url);
        //unsafe { crate::oauth::actions::oauth_open_window(&resp.url, "oauth"); }
    }
}
pub async fn finalize(req: CreateSessionOAuthRequest, url_kind: OAuthUrlKind) {
    if let Ok(resp) = api_no_auth_with_credentials::<CreateSessionResponse, EmptyError, _>(
        CreateOAuth::PATH,
        CreateOAuth::METHOD,
        Some(req),
    )
    .await
    {
        match resp {
            CreateSessionResponse::Login(resp) => {
                crate::login::actions::do_success(&resp.csrf);
            }
            CreateSessionResponse::Register {
                response,
                oauth_profile,
            } => {
                let csrf = response.csrf;

                match url_kind {
                    OAuthUrlKind::Register => {
                        storage::save_csrf_token(&csrf);
                        let route: String =
                            Route::User(UserRoute::ContinueRegistration(oauth_profile)).into();
                        dominator::routing::go_to_url(&route);
                    }
                    OAuthUrlKind::Login => {
                        let _ = web_sys::window()
                            .unwrap_ji()
                            .alert_with_message(crate::strings::STR_AUTH_OAUTH_LOGIN_FAIL);
                        let route: String = Route::User(UserRoute::Register).into();
                        dominator::routing::go_to_url(&route);
                    }
                }
            }
        }
    }
}
