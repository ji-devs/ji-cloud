use super::state::*;
use dominator::clone;
use shared::{
    api::endpoints::{user, ApiEndpoint},
    domain::{session::*, user::CreateUserRequest},
    error::EmptyError,
};
use std::rc::Rc;
use utils::prelude::*;

const STR_EMAIL_IN_USE: &str = "An account is already set up with this email address";
const STR_EMAIL_INVALID: &str = "Invalid email address";
const STR_EMAIL_EMPTY: &str = "Email can't be empty";

impl RegisterStart {
    pub fn register_email(self: &Rc<Self>) {
        let state = self;

        state.tried_to_submit.set(true);

        let email = state.email.borrow().clone();
        let password = state.password.get_value();

        if !state.password.password_acceptable() || state.email_error.lock_ref().is_some() {
            return;
        }

        state.loader.load(clone!(state => async move {
            let query = CreateUserRequest {
                email: email.clone(),
                password
            };

            let (resp, status):(Result<(), EmptyError>, u16) = api_no_auth_empty_status(user::Create::PATH, user::Create::METHOD, Some(query)).await;

            match resp {
                Ok(_) => {
                    let route:String = Route::User(UserRoute::SendEmailConfirmation(email)).into();
                    dominator::routing::go_to_url(&route);
                },
                Err(_) => {
                    if status == 409 {
                        state.email_error.set(Some(STR_EMAIL_IN_USE));
                    } else {
                        state.email_error.set(Some(STR_EMAIL_INVALID));
                    }
                }
            }
        }));
    }

    pub fn register_google(self: &Rc<Self>) {
        self.loader.load(async {
            crate::oauth::actions::redirect(GetOAuthUrlServiceKind::Google, OAuthUrlKind::Register)
                .await;
        });
    }

    pub fn update_email(self: &Rc<Self>, email: String) {
        let error = if email.is_empty() {
            Some(STR_EMAIL_EMPTY)
        } else if !email.contains('@') {
            Some(STR_EMAIL_INVALID)
        } else {
            None
        };
        self.email_error.set(error);

        *self.email.borrow_mut() = email;
    }
}
