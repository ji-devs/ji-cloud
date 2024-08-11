use std::rc::Rc;
use utils::routes::WelcomeParams;
// use shared::domain::billing::PlanType;
// use utils::prelude::get_user_mutable;

pub struct Welcome {
    // pub plan_type: PlanType
    pub subscribed: bool,
}

impl Welcome {
    pub fn new(params: WelcomeParams) -> Rc<Self> {
        // get_user_mutable().lock_ref().
        Rc::new(Self {
            // plan_type,
            subscribed: params.subscribed,
        })
    }
}
