use std::rc::Rc;

use dominator::clone;
use futures::join;
use futures_signals::signal::Mutable;
use gloo_timers::future::TimeoutFuture;
use shared::domain::billing::{
    CreateCustomerPortalLinkPath, PlanType, SubscriptionPauseRequest, SubscriptionStatus,
    UpdateSubscriptionPausedPath,
};
use shared::{
    api::endpoints::{self, meta, user},
    domain::{
        billing::{
            Account, AccountIfAuthorized, CancellationStatus, GetSchoolAccountResponse,
            IndividualAccountPath, IndividualAccountResponse, SchoolAccountPath,
            SubscriptionCancellationStatusRequest, UpdateSubscriptionCancellationPath,
            UpgradeSubscriptionPlanPath, UpgradeSubscriptionPlanRequest,
        },
        meta::GetMetadataPath,
        user::{GetProfilePath, PatchProfilePath, ResetPasswordPath, ResetPasswordRequest},
    },
};
use utils::toasts;
use wasm_bindgen_futures::spawn_local;

use super::state::{IndividualOrSchool, PlanSectionInfo, ResetPasswordStatus, SettingsPage};
use utils::{bail_on_err, prelude::*, unwrap::UnwrapJiExt};

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
                        .ok()
                        .map(|info| Rc::new(info)),
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

                self.plan_info.set(
                    (resp, IndividualOrSchool::Individual)
                        .try_into()
                        .ok()
                        .map(|info| Rc::new(info)),
                );
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

            toasts::notice(if auto_renew {
                "Auto-renewal enabled"
            } else {
                "Auto-renewal disabled"
            })
        }));
    }

    pub fn set_paused(self: &Rc<Self>, paused: bool) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.plan_info.replace_with(|plan_info| {
                if let Some(section_info) = plan_info {
                    let mut section_info = (**section_info).clone();
                    section_info.status = if paused {
                        SubscriptionStatus::Paused
                    } else {
                        SubscriptionStatus::Active
                    };

                    Some(Rc::new(section_info))
                } else {
                    plan_info.clone()
                }
            });

            let req = SubscriptionPauseRequest { paused };

            let _ = endpoints::billing::UpdateSubscriptionPaused::api_with_auth(UpdateSubscriptionPausedPath(), Some(req)).await.toast_on_err();

            toasts::notice(if paused {
                "Subscription paused"
            } else {
                "Subscription resumed"
            })
        }));
    }

    pub fn change_to(self: &Rc<Self>, new_plan_type: PlanType) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let req = UpgradeSubscriptionPlanRequest {
                plan_type: new_plan_type.clone(),
                promotion_code: None,
            };

            let res = endpoints::billing::UpgradeSubscriptionPlan::api_with_auth(
                UpgradeSubscriptionPlanPath(),
                Some(req),
            )
            .await
            .toast_on_err();
            let _ = bail_on_err!(res);

            state.load_account().await;

            toasts::success("Plan updated successfully!");

            get_user_mutable().replace_with(clone!(new_plan_type => move |user| {
                let mut user = user.clone();
                if let Some(user) = &mut user {
                    if let Some(account_summary) = &mut user.account_summary {
                        if let Some(plan_type) = &mut account_summary.plan_type {
                            *plan_type = new_plan_type;
                        }
                    }
                }
                user
            }));
        }));
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

    pub fn load_portal_link(self: &Rc<Self>) {
        let state = self;
        if state.portal_link.lock_ref().is_none() {
            state.loader.load(clone!(state => async move {
                let session_url = endpoints::billing::CreateCustomerPortalLink::api_with_auth(
                    CreateCustomerPortalLinkPath(),
                    None,
                ).await.toast_on_err();
                let session_url: String = bail_on_err!(session_url);

                state.portal_link.set(Some(session_url));
            }));
        }
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
        let payment_method_type = account
            .payment_method
            .map(|method| method.payment_method_type);
        Ok(Self {
            auto_renew: Mutable::new(subscription.auto_renew),
            status: subscription.status,
            is_trial: subscription.is_trial,
            payment_method_type,
            price: subscription.price,
            coupon: subscription.applied_coupon,
            current_period_end: subscription.current_period_end,
            individual_or_school,
        })
    }
}
