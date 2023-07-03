use std::rc::Rc;

// use shared::domain::billing::PlanType;
// use utils::prelude::get_user_mutable;

pub struct Welcome {
    // pub plan_type: PlanType
}

impl Welcome {
    pub fn new() -> Rc<Self> {
        // get_user_mutable().lock_ref().
        Rc::new(Self {
            // plan_type,
        })
    }
}
