use super::state::*;
use dominator::clone;
use shared::{
    api::endpoints::user,
    domain::user::{ChangePasswordPath, ChangePasswordRequest},
};
use std::rc::Rc;
use utils::prelude::*;

const STR_ERROR_RESETTING: &str = "Error resetting your password, please try again";

impl PasswordResetPage {
    pub fn change_password(self: &Rc<Self>) {
        let state = self;

        state.tried_to_submit.set(true);

        if !state.password.password_acceptable() {
            return;
        }

        state.loader.load(clone!(state => async move {
            state.tried_to_submit.set(true);
            let password:String = state.password.get_value();
            let query = ChangePasswordRequest::Change {
                token: state.token.clone(),
                password,
                force_logout: true
            };

            let (resp, _status):(anyhow::Result<()>, u16) = user::ChangePassword::api_no_auth_empty_status(ChangePasswordPath(), Some(query)).await;

            match resp {
                Ok(_) => {
                    let route:String = Route::User(UserRoute::Login(Default::default())).into();
                    dominator::routing::go_to_url(&route);
                },
                Err(_err) => {
                    state.password.set_error(STR_ERROR_RESETTING);
                }
            }
        }));
    }
}
