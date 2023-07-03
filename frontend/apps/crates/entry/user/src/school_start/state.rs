use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::billing::PlanType;
use std::rc::Rc;
use utils::{prelude::get_user_mutable, unwrap::UnwrapJiExt};

pub struct SchoolStart {
    pub plan_type: PlanType,
    pub name: Mutable<String>,
    pub location: Mutable<Option<serde_json::Value>>,
    pub loader: AsyncLoader,
    pub tried_to_submit: Mutable<bool>,
}
impl SchoolStart {
    pub fn new(plan_type: PlanType) -> Rc<Self> {
        let user = get_user_mutable();
        let user = user.lock_ref();
        let user = user.as_ref().unwrap_ji();
        Rc::new(Self {
            plan_type,
            loader: AsyncLoader::new(),
            name: Mutable::new(user.organization.clone().unwrap_or_default()),
            location: Mutable::new(user.location.clone()),
            tried_to_submit: Default::default(),
        })
    }
}
