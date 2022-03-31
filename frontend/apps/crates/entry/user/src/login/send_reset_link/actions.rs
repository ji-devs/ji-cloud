use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::user::ResetPasswordRequest};
use utils::prelude::ApiEndpointExt;

use super::SendResetLink;

impl SendResetLink {
    pub(super) fn submit(self: &Rc<Self>) {
        let state = self;

        let email = state.email.get_value();

        state.loader.load(clone!(state => async move {
            let req = ResetPasswordRequest {
                email
            };

            let res = endpoints::user::ResetPassword::api_no_auth_empty(Some(req)).await;

            match res {
                Ok(_) => {
                    state.reset_sent.set(true);
                },
                Err(_err) => {
                    todo!()
                }
            }
        }));
    }
}
