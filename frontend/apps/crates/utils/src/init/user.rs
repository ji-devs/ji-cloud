use crate::{fetch::*, unwrap::UnwrapJiExt};
use futures_signals::signal::{Mutable, Signal};
use once_cell::sync::OnceCell;
use shared::{
    api::endpoints::user::Profile,
    domain::{
        billing::{PlanType, SchoolId},
        user::{GetProfilePath, UserId, UserProfile},
    },
};

static USER: OnceCell<Mutable<Option<UserProfile>>> = OnceCell::new();

pub(crate) async fn init() {
    let (result, status) = Profile::api_with_auth_status(GetProfilePath(), None).await;

    // `USER` is private and the only way to initialize it is through `init` - `set()`
    // should never fail at this point.
    match result {
        Ok(user) if status != 401 || status != 403 => {
            let _ = USER.set(Mutable::new(Some(user)));
        }
        _ => {
            let _ = USER.set(Mutable::new(None));
        }
    }
}

pub fn get_user_mutable() -> Mutable<Option<UserProfile>> {
    USER.get().cloned().unwrap_ji()
}

pub fn get_user_cloned() -> Option<UserProfile> {
    get_user_mutable().get_cloned()
}

pub fn is_user_set() -> bool {
    get_user_mutable().lock_ref().is_some()
}

pub fn get_user_id() -> Option<UserId> {
    get_user_mutable().lock_ref().as_ref().map(|user| user.id)
}

pub fn get_school_id() -> Option<SchoolId> {
    let school_id = get_user_mutable()
        .lock_ref()
        .as_ref()?
        .account_summary
        .as_ref()?
        .school_id?;
    Some(school_id)
}

pub fn get_plan_type() -> Option<PlanType> {
    let plan = get_user_mutable()
        .lock_ref()
        .as_ref()?
        .account_summary
        .as_ref()?
        .plan_type?;
    Some(plan)
}

pub fn plan_type_signal() -> impl Signal<Item = Option<PlanType>> {
    get_user_mutable().signal_ref(|user| -> Option<PlanType> {
        let plan = user.as_ref()?.account_summary.as_ref()?.plan_type?;
        Some(plan)
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlanTier {
    Pro,
    Basic,
    Free,
}

pub fn get_plan_tier() -> PlanTier {
    match get_plan_type() {
        Some(
            PlanType::IndividualProMonthly
            | PlanType::IndividualProAnnually
            | PlanType::SchoolLevel1
            | PlanType::SchoolLevel2
            | PlanType::SchoolLevel3
            | PlanType::SchoolLevel4
            | PlanType::SchoolUnlimited,
        ) => PlanTier::Pro,
        Some(PlanType::IndividualBasicMonthly | PlanType::IndividualBasicAnnually) => {
            PlanTier::Basic
        }
        None => PlanTier::Free,
    }
}
