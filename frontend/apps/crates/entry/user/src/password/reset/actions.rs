use super::{
    state::*,
    super::state::*
};
use std::rc::Rc;
use dominator::clone;
use shared::{
    api::endpoints::{ApiEndpoint, user},
    domain::{
        user::ChangePasswordRequest,
        session::NewSessionResponse,
    },
    error::EmptyError
};
use utils::{prelude::*, storage};

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

        let (resp, status):(Result<(), EmptyError>, u16) = api_no_auth_empty_status(&user::ChangePassword::PATH, user::ChangePassword::METHOD, Some(query)).await;

        
        match resp {
            Ok(_) => {
                let route:String = Route::User(UserRoute::Login(String::new())).into();
                dominator::routing::go_to_url(&route);
            }, 
            Err(err) => {
                state.password.status.set(Some(PasswordStatus::ResetError));
            }
        }
    }));
}
