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


//Called from the oauth popup - posts a message back to the _opener_ window
pub async fn finalize(req: CreateSessionOAuthRequest) {
    
    if let Ok(resp) = api_no_auth::<CreateSessionOAuthResponse, EmptyError, _>(&CreateOAuth::PATH, CreateOAuth::METHOD, Some(req)).await {
        let window:Window = web_sys::window() 
            .unwrap_throw();

        let parent:Window = window
            .opener()
            .unwrap_throw()
            .unchecked_into();

        let domain = window.location().origin().unwrap_throw();

        let msg = serde_wasm_bindgen::to_value(&resp).unwrap_throw();

        parent.post_message(&msg, &domain);

        window.close();
    }

}
