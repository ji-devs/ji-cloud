use dominator_helpers::futures::AsyncLoader;
use shared::domain::billing::PlanType;
use std::rc::Rc;
use utils::routes::StripeRedirectParams;

pub struct Subscribe2 {
    pub plan_type: PlanType,
    pub params: Option<StripeRedirectParams>,
    pub loader: AsyncLoader,
    pub promo: Option<String>,
}
impl Subscribe2 {
    pub fn new(
        plan_type: PlanType,
        params: Option<StripeRedirectParams>,
        promo: Option<String>,
    ) -> Rc<Self> {
        Rc::new(Self {
            plan_type,
            params,
            loader: AsyncLoader::new(),
            promo,
        })
    }
}
