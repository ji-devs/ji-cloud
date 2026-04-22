use shared::{api::endpoints, domain::session::*};
use utils::{prelude::ApiEndpointExt, routes::*, storage, unwrap::UnwrapJiExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/oauth.js")]
extern "C" {
    pub fn oauth_open_window(url: &str, name: &str);
}

pub async fn redirect(service_kind: GetOAuthUrlServiceKind, url_kind: OAuthUrlKind) {
    if let Ok(resp) =
        endpoints::session::GetOAuthUrl::api_no_auth(GetOAuthPath(service_kind, url_kind), None)
            .await
    {
        let _ = web_sys::window().unwrap_ji().location().set_href(&resp.url);
        //unsafe { crate::oauth::actions::oauth_open_window(&resp.url, "oauth"); }
    }
}

pub async fn finalize(data: OauthData, redirect_kind: OAuthUrlKind) {
    let req = match data {
        OauthData::Google(code) => CreateSessionOAuthRequest::Google {
            code,
            redirect_kind,
        },
    };

    let (res, status) = endpoints::session::CreateOAuth::api_no_auth_with_credentials_status(
        CreateSessionOAuthPath(),
        Some(req),
    )
    .await;

    match res {
        Ok(res) => match res {
            CreateSessionResponse::Login(resp) => {
                crate::login::actions::do_success(&resp.csrf);
            }
            CreateSessionResponse::Register {
                response,
                oauth_profile,
                needs_email_verification,
            } => {
                let csrf = response.csrf;
                storage::save_csrf_token(&csrf);
                if needs_email_verification {
                    let email = oauth_profile
                        .as_ref()
                        .map(|p| p.email.clone())
                        .unwrap_or_default();
                    let route = Route::User(UserRoute::SendEmailConfirmation(email)).to_string();
                    dominator::routing::go_to_url(&route);
                } else {
                    let route =
                        Route::User(UserRoute::ContinueRegistration(oauth_profile)).to_string();
                    dominator::routing::go_to_url(&route);
                }
            }
        },
        Err(_) => match status {
            409 => {
                let route =
                    Route::User(UserRoute::Login(LoginQuery::basic_tried_oauth())).to_string();
                dominator::routing::go_to_url(&route);
            }
            _ => {
                todo!();
            }
        },
    }
}
