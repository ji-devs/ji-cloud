use std::rc::Rc;

use dominator::clone;
use futures::join;
use futures_signals::signal::Mutable;
use gloo_timers::future::TimeoutFuture;
use shared::{
    api::endpoints::{self, meta, user},
    domain::{
        billing::{
            Account, AccountIfAuthorized, CancellationStatus, GetSchoolAccountResponse,
            IndividualAccountPath, IndividualAccountResponse, SchoolAccountPath,
            SubscriptionCancellationStatusRequest, UpdateSubscriptionCancellationPath,
        },
        meta::GetMetadataPath,
        user::{GetProfilePath, PatchProfilePath, ResetPasswordPath, ResetPasswordRequest},
    },
};
use wasm_bindgen_futures::spawn_local;

use super::state::{IndividualOrSchool, PlanSectionInfo, ResetPasswordStatus, SettingsPage};
use utils::{prelude::*, toasts, unwrap::UnwrapJiExt};

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
            join!(
                state.load_profile(), // TODO: use utils::init::user
                state.load_metadata(),
                state.load_account(),
            );
        }));
    }

    async fn load_account(self: &Rc<Self>) {
        match account_type_to_fetch() {
            Some(IndividualOrSchool::School(school_id)) => {
                let resp = endpoints::account::GetSchoolAccount::api_with_auth(
                    SchoolAccountPath(school_id),
                    None,
                )
                .await
                .toast_on_err()
                .unwrap_ji();

                self.plan_info.set(
                    (resp, IndividualOrSchool::School(school_id))
                        .try_into()
                        .ok(),
                );
            }
            Some(IndividualOrSchool::Individual) => {
                let resp = endpoints::account::GetIndividualAccount::api_with_auth(
                    IndividualAccountPath(),
                    None,
                )
                .await
                .toast_on_err()
                .unwrap_ji();

                self.plan_info
                    .set((resp, IndividualOrSchool::Individual).try_into().ok());
            }
            None => {}
        }
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

    pub fn set_auto_renew(self: &Rc<Self>, auto_renew: bool) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.plan_info.replace_with(|plan_info| {
                if let Some(plan_info) = plan_info {
                    plan_info.auto_renew.set_neq(auto_renew);
                }
                plan_info.clone()
            });

            let req = SubscriptionCancellationStatusRequest {
                status: match auto_renew {
                    true => CancellationStatus::RemoveCancellation,
                    false => CancellationStatus::CancelAtPeriodEnd,
                }
            };

            let _ = endpoints::billing::UpdateSubscriptionCancellation::api_with_auth(UpdateSubscriptionCancellationPath(), Some(req)).await.toast_on_err();
        }));
    }

    pub fn change_to_annual_billing(self: &Rc<Self>) {
        toasts::error("Not yet implemented.");
        // TODO:
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

fn account_type_to_fetch() -> Option<IndividualOrSchool> {
    let user = get_user_mutable();
    let user = user.lock_ref();
    let user = user.as_ref()?;

    let account_summary = match &user.account_summary {
        Some(account_summary) => account_summary,
        None => return None,
    };

    match account_summary.school_id {
        Some(school_id) => {
            if account_summary.is_admin {
                Some(IndividualOrSchool::School(school_id))
            } else {
                None
            }
        }
        None => Some(IndividualOrSchool::Individual),
    }
}

impl TryFrom<(IndividualAccountResponse, IndividualOrSchool)> for PlanSectionInfo {
    type Error = ();

    fn try_from(
        (res, individual_or_school): (IndividualAccountResponse, IndividualOrSchool),
    ) -> Result<Self, Self::Error> {
        let account = res.account.ok_or(())?;
        (account, individual_or_school).try_into()
    }
}

impl TryFrom<(GetSchoolAccountResponse, IndividualOrSchool)> for PlanSectionInfo {
    type Error = ();

    fn try_from(
        (res, individual_or_school): (GetSchoolAccountResponse, IndividualOrSchool),
    ) -> Result<Self, Self::Error> {
        let account = match res.account {
            AccountIfAuthorized::Unauthorized => {
                return Err(());
            }
            AccountIfAuthorized::Authorized(account) => account,
        };
        (account, individual_or_school).try_into()
    }
}

impl TryFrom<(Account, IndividualOrSchool)> for PlanSectionInfo {
    type Error = ();

    fn try_from(
        (account, individual_or_school): (Account, IndividualOrSchool),
    ) -> Result<Self, Self::Error> {
        let subscription = account.subscription.ok_or(())?;
        let payment_method = account.payment_method.ok_or(())?;
        Ok(Self {
            auto_renew: Mutable::new(subscription.auto_renew),
            payment_method_type: payment_method.payment_method_type,
            price: 0,
            current_period_end: subscription.current_period_end,
            individual_or_school,
        })
    }
}
