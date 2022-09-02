use super::state::*;
use shared::{
    api::endpoints::user,
    domain::{
        session::NewSessionResponse,
        user::{VerifyEmailPath, VerifyEmailRequest},
    },
};
use utils::{prelude::*, storage};

impl VerifyEmailPage {
    pub async fn verify(&self) {
        let query = VerifyEmailRequest::Verify {
            token: self.token.clone(),
        };

        let resp: anyhow::Result<Option<NewSessionResponse>> =
            user::VerifyEmail::api_no_auth_with_credentials(VerifyEmailPath(), Some(query)).await;

        match resp {
            Ok(resp) => match resp {
                Some(resp) => {
                    let csrf = resp.csrf;

                    storage::save_csrf_token(&csrf);
                    let route: String = Route::User(UserRoute::ContinueRegistration(None)).into();
                    dominator::routing::go_to_url(&route);
                }
                None => {
                    log::error!("Got error!")
                }
            },
            Err(_) => {
                log::error!("Got error!")
            }
        }
    }
}
