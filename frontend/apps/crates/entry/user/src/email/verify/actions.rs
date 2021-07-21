use super::state::*;
use shared::{
    api::endpoints::{ApiEndpoint, user},
    domain::{
        user::VerifyEmailRequest,
        session::NewSessionResponse,
    },
    error::EmptyError
};
use utils::{prelude::*, storage};

impl VerifyEmailPage {
    pub async fn verify(&self) {
        let query = VerifyEmailRequest::Verify {
            token: self.token.clone() 
        };

        let resp:Result<Option<NewSessionResponse>, EmptyError> = api_no_auth_with_credentials(&user::VerifyEmail::PATH, user::VerifyEmail::METHOD, Some(query)).await;

        
        match resp {
            Ok(resp) => {
                match resp {
                    Some(resp) => {
                        let csrf = resp.csrf;

                        storage::save_csrf_token(&csrf);
                        let route:String = Route::User(UserRoute::ContinueRegistration).into();
                        dominator::routing::go_to_url(&route);
                    },
                    None => {
                        log::error!("Got error!")
                    }
                }
            }, 
            Err(err) => {
                log::error!("Got error!")
            }
        }
    }
}
