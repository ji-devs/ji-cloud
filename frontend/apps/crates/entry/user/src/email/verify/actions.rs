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

impl VerifyEmailPage {
    pub fn verify(&self) {

        self.loader.load(async move {
            //TODO
        });
    }
}
