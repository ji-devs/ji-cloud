use super::state::*;
use shared::{
    api::endpoints::user,
    domain::user::{VerifyEmailPath, VerifyEmailRequest},
};
use utils::prelude::*;

impl SendEmailConfirmationPage {
    pub fn resend(&self) {
        let mode = self.mode.clone();
        let email = self.email.clone();

        self.loader.load(async move {
            let query = VerifyEmailRequest::Resend { email };

            let resp: anyhow::Result<()> =
                user::VerifyEmail::api_no_auth_empty(VerifyEmailPath(), Some(query)).await;

            match resp {
                Ok(_) => {
                    mode.set_neq(Mode::Sent);
                }
                Err(_err) => {
                    log::error!("Got error!")
                }
            }
        });
    }
}
