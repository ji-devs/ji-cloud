use futures_signals::signal::Mutable;
use shared::domain::billing::{
    AmountInCents, BillingInterval, PlanId, SubscriptionPlanDetailsResponse,
    SubscriptionPlansResponse, SubscriptionType, TrialPeriod,
};
use std::rc::Rc;
use utils::routes::SubscribeRedirect;

pub struct Subscribe {
    pub plans: Mutable<Option<Rc<SubscriptionPlansResponse>>>,
    pub billing_interval: Mutable<BillingInterval>,
    pub subscription_type: Mutable<SubscriptionType>,
    pub selected_plan: Mutable<Option<SubscriptionPlanDetailsResponse>>,
    pub subscribe_state: Mutable<SubscribeState>,
}

impl Subscribe {
    pub fn new(
        plans: Mutable<Option<Rc<SubscriptionPlansResponse>>>,
        redirect: Option<SubscribeRedirect>,
    ) -> Rc<Self> {
        Rc::new(Self {
            plans,
            billing_interval: Mutable::new(BillingInterval::Monthly),
            subscription_type: Mutable::new(SubscriptionType::Individual),
            selected_plan: Default::default(),
            subscribe_state: Mutable::new(match redirect {
                Some(redirect) => SubscribeState::CompleteSubscription(redirect),
                None => SubscribeState::default(),
            }),
        })
    }
}

#[derive(Debug, Clone)]
pub enum SubscribeState {
    Select,
    PaymentMethod(SelectedSubscription),
    CompleteSubscription(SubscribeRedirect),
}

impl Default for SubscribeState {
    fn default() -> Self {
        Self::Select
    }
}

#[derive(Debug, Clone)]
pub struct SelectedSubscription {
    pub billing_interval: BillingInterval,
    pub subscription_type: SubscriptionType,
    pub plan_id: PlanId,
    pub amount_in_cents: AmountInCents,
    pub trial_period: Option<TrialPeriod>,
}

impl SelectedSubscription {
    pub fn new(plan: SubscriptionPlanDetailsResponse) -> Self {
        Self {
            billing_interval: plan.billing_interval,
            subscription_type: plan.subscription_type,
            plan_id: plan.plan_id,
            amount_in_cents: plan.amount_in_cents,
            trial_period: plan.trial_period,
        }
    }
}
