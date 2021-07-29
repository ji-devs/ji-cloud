use super::state::*;
use shared::{
    api::endpoints::{ApiEndpoint, user},
    domain::{
        user::VerifyEmailRequest,
        session::NewSessionResponse,
    },
    error::EmptyError
};
use utils::prelude::*;

impl SendEmailConfirmationPage {
    pub fn resend(&self) {
        let mode = self.mode.clone();
        let email = self.email.clone();

        self.loader.load(async move {

            let query = VerifyEmailRequest::Resend {
                email
            };

            let resp:Result<(), EmptyError> = api_no_auth_empty(&user::VerifyEmail::PATH, user::VerifyEmail::METHOD, Some(query)).await;

            
            match resp {
                Ok(_) => {
                    mode.set_neq(Mode::Sent);
                }, 
                Err(err) => {
                    log::error!("Got error!")
                }
            }
        });
    }
}
