use std::rc::Rc;

use dominator::clone;
use futures::future::join;
use gloo_timers::future::TimeoutFuture;
use shared::{
    api::endpoints::{self, meta, user},
    domain::{
        meta::GetMetadataPath,
        user::{GetProfilePath, PatchProfilePath, ResetPasswordPath, ResetPasswordRequest},
    },
};
use wasm_bindgen_futures::spawn_local;

use super::state::{ResetPasswordStatus, SettingsPage};
use utils::{prelude::*, unwrap::UnwrapJiExt};

impl SettingsPage {
    pub fn send_reset_password(self: &Rc<Self>) {
        let state = self;

        state
            .reset_password_status
            .set(ResetPasswordStatus::Loading);

        spawn_local(clone!(state => async move {
            let req = ResetPasswordRequest {
                email: state.user.email.get_cloned()
            };

            let res = endpoints::user::ResetPassword::api_no_auth(ResetPasswordPath(), Some(req)).await;

            match res {
                Ok(_) => {
                    state.reset_password_status.set(ResetPasswordStatus::Sent);
                },
                Err(_err) => {
                    todo!()
                }
            }
            TimeoutFuture::new(5000).await;
            state.reset_password_status.set(ResetPasswordStatus::default());
        }));
    }

    pub fn load_initial_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join(
                state.load_profile(),
                state.load_metadata()
            ).await;
        }));
    }

    async fn load_profile(self: &Rc<Self>) {
        //let resp:Result<UserProfile, EmptyError> = api_with_auth::< _, _, ()>(&user::Profile::PATH, user::Profile::METHOD, None).await;
        let resp = user::Profile::api_with_auth(GetProfilePath(), None).await;

        self.user.fill_from_user(resp.unwrap_ji());
    }

    async fn load_metadata(self: &Rc<Self>) {
        match meta::Get::api_with_auth(GetMetadataPath(), None).await {
            Err(_) => {}
            Ok(res) => {
                self.metadata.set(Some(res));
            }
        };
    }

    pub fn save_profile(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let info = state.user.to_update();

            let res = user::PatchProfile::api_with_auth(PatchProfilePath(), Some(info)).await;
            if let Err(_err) = res {
                todo!()
            }
        }));
    }
}
