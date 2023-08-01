use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::billing::PlanType;
use std::{cell::RefCell, rc::Rc};

use super::stripe::Stripe;

pub struct Subscribe1 {
    pub plan_type: PlanType,
    pub loader: AsyncLoader,
    pub stripe_client_secret: Mutable<Option<String>>,
    pub(super) stripe: RefCell<Option<Stripe>>,
    pub promo: Option<String>,
    pub pay_with_check: Mutable<bool>,
}
impl Subscribe1 {
    pub fn new(plan_type: PlanType, promo: Option<String>) -> Rc<Self> {
        Rc::new(Self {
            plan_type,
            loader: AsyncLoader::new(),
            stripe_client_secret: Mutable::new(None),
            stripe: RefCell::new(None),
            promo,
            pay_with_check: Mutable::new(false),
        })
    }
}
