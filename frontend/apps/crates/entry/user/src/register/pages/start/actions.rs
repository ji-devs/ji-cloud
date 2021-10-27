use super::state::*;
use dominator::clone;
use shared::{
    api::endpoints::{user, ApiEndpoint},
    domain::{session::*, user::CreateUserRequest},
    error::EmptyError,
};
use std::rc::Rc;
use utils::prelude::*;

use crate::password::state::*;

pub fn register_email(state: Rc<State>) {
    state.clear_email_status();
    state.password.clear_status();

    let mut early_exit = false;

    if state.password.value.borrow().len() < 6 {
        state.password.status.set(Some(PasswordStatus::PwShort));
        early_exit = true;
    }

    let email: String = state.email.borrow().clone();
    let password: String = state.password.value.borrow().clone();

    if email.is_empty() {
        state.email_status.set(Some(EmailStatus::EmptyEmail));
        early_exit = true;
    }

    if early_exit {
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
            Err(_err) => {
                if status == 409 {
                    state.email_status.set(Some(EmailStatus::EmailExists));
                } else {
                    state.email_status.set(Some(EmailStatus::InvalidEmail));
                }
            }
        }
    }));
}

pub fn register_google(state: Rc<State>) {
    state.clear_email_status();
    state.password.clear_status();

    state.loader.load(async {
        crate::oauth::actions::redirect(GetOAuthUrlServiceKind::Google, OAuthUrlKind::Register)
            .await;
    });
}
