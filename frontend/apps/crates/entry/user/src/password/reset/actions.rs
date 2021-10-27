use super::{super::state::*, state::*};
use dominator::clone;
use shared::{
    api::endpoints::{user, ApiEndpoint},
    domain::user::ChangePasswordRequest,
    error::EmptyError,
};
use std::rc::Rc;
use utils::prelude::*;

pub fn change_password(state: Rc<PasswordResetPage>) {
    state.password.clear_status();

    if state.password.strength.get() != PasswordStrength::Strong {
        state.password.status.set(Some(PasswordStatus::PwWeak));
        return;
    }

    state.loader.load(clone!(state => async move {
        let password:String = state.password.value.borrow().clone();
        let query = ChangePasswordRequest::Change {
            token: state.token.clone(),
            password,
            force_logout: true
        };

        let (resp, _status):(Result<(), EmptyError>, u16) = api_no_auth_empty_status(user::ChangePassword::PATH, user::ChangePassword::METHOD, Some(query)).await;

        match resp {
            Ok(_) => {
                let route:String = Route::User(UserRoute::Login(String::new())).into();
                dominator::routing::go_to_url(&route);
            },
            Err(_err) => {
                state.password.status.set(Some(PasswordStatus::ResetError));
            }
        }
    }));
}
