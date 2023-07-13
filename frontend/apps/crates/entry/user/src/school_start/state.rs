use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::billing::PlanType;
use std::rc::Rc;
use utils::{
    prelude::get_user_mutable,
    routes::{Route, UserRoute},
};

pub struct SchoolStart {
    pub plan_type: PlanType,
    pub name: Mutable<String>,
    pub location: Mutable<Option<serde_json::Value>>,
    pub loader: AsyncLoader,
    pub tried_to_submit: Mutable<bool>,
}
impl SchoolStart {
    /// Redirecting and returning None if the use is not logged in
    pub fn new(plan_type: PlanType) -> Option<Rc<Self>> {
        let user = get_user_mutable();
        let user = user.lock_ref();
        let user = if let Some(user) = &*user {
            user
        } else {
            Route::User(UserRoute::NoAuth).redirect();
            return None;
        };

        Some(Rc::new(Self {
            plan_type,
            loader: AsyncLoader::new(),
            name: Mutable::new(user.organization.clone().unwrap_or_default()),
            location: Mutable::new(user.location.clone()),
            tried_to_submit: Default::default(),
        }))
    }
}
