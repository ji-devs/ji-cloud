use dominator_helpers::futures::AsyncLoader;
use shared::domain::billing::PlanType;
use std::rc::Rc;

pub struct Subscribe2 {
    pub plan_type: PlanType,
    pub loader: AsyncLoader,
}
impl Subscribe2 {
    pub fn new(plan_type: PlanType) -> Rc<Self> {
        Rc::new(Self {
            plan_type,
            loader: AsyncLoader::new(),
        })
    }
}
