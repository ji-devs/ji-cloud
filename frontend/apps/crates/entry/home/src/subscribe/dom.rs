use crate::subscribe::state::{SelectedSubscription, Subscribe, SubscribeState};
use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use shared::api::endpoints::billing::{CreateSetupIntent, CreateSubscription};
use shared::domain::billing::{
    AccountLimit, BillingInterval, CreateSetupIntentPath, CreateSubscriptionPath,
    CreateSubscriptionRequest, PlanId, SubscriptionPlanDetailsResponse, SubscriptionPlansResponse,
    SubscriptionType,
};
use std::collections::HashMap;
use std::rc::Rc;
use utils::{events, prelude::*};

impl Subscribe {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("empty-fragment", {
            .child_signal(state.subscribe_state.signal_cloned().map(clone!(state => move |subscribe_state| {
                Some(match subscribe_state {
                    SubscribeState::Select => Rc::clone(&state).render_plan_selection(),
                    SubscribeState::PaymentMethod(subscription) => Rc::clone(&state).render_payment_details(subscription),
                    SubscribeState::CompleteSubscription(redirect) => {
                        let is_subscribed: Mutable<Option<Result<(), String>>> = Mutable::new(None);

                        let signal = map_ref! {
                            let is_subscribed = is_subscribed.signal_cloned(),
                            let plans = state.plans.signal_cloned()
                                => {
                                (is_subscribed.clone(), plans.clone())
                            }
                        };

                        html!("empty-fragment", {
                            .future(clone!(state => async move {
                                let result: Result<_, anyhow::Error> = CreateSubscription::api_with_auth(
                                    CreateSubscriptionPath(),
                                    Some(CreateSubscriptionRequest {
                                        setup_intent_id: redirect.setup_intent_redirect_params.setup_intent,
                                        plan_id: redirect.plan_id,
                                    })
                                ).await;

                                is_subscribed.set(match result {
                                    Ok(subscription) => Some(Ok(())),
                                    Err(error) => {
                                        Some(Err("Unable to create subscription".to_string()))
                                    }
                                });
                            }))
                            .child_signal(signal.map(clone!(state => move |(is_subscribed, plans)| {
                                match (is_subscribed, plans) {
                                    (Some(is_subscribed), Some(plans)) => {
                                        let plan = plans.plans.get(&redirect.plan_id).unwrap_ji();
                                        match is_subscribed {
                                            Ok(_) => {
                                                Some(html!("div", {
                                                    .text(&format!(
                                                        "Subscribed to plan ID: {:?}, Amount in cents: {:?}, Trial period: {:?}",
                                                        plan.plan_id,
                                                        plan.amount_in_cents,
                                                        plan.trial_period,
                                                    ))
                                                }))
                                            }
                                            Err(error) => {
                                                Some(html!("div", {
                                                    .text(&format!(
                                                        "Could not subscribe to plan ID: {:?}: {error:?}",
                                                        plan.plan_id,
                                                    ))
                                                }))
                                            }
                                        }
                                    },
                                    _ => None // TODO handle loading
                                }
                            })))
                        })
                    },
                })
            })))
        })
    }

    fn render_plan_selection(self: Rc<Self>) -> Dom {
        let state = self;

        let subscription_type_signal = map_ref! {
            let plans = state.plans.signal_cloned(),
            let subscription_type = state.subscription_type.signal_cloned()
                => {
                plans.as_ref().map(|plans| (*subscription_type, plans.clone()))
            }
        };

        html!("div", {
            .child(html!("button", {
                .text_signal(state.subscription_type.signal_cloned().map(clone!(state => move |subscription_type| {
                    match subscription_type {
                        SubscriptionType::Individual => "Individual (switch to School)",
                        SubscriptionType::School => "School (switch to Individual)",
                    }
                })))
                .event(clone!(state => move |_evt: events::Click| {
                    let subscription_type = state.subscription_type.get_cloned();

                    match subscription_type {
                        SubscriptionType::Individual => state.subscription_type.set(SubscriptionType::School),
                        SubscriptionType::School => state.subscription_type.set(SubscriptionType::Individual),
                    }
                }))
            }))
            .child(html!("button", {
                .text_signal(state.billing_interval.signal_cloned().map(clone!(state => move |billing_interval| {
                    match billing_interval {
                        BillingInterval::Monthly => "Monthly (switch to Annual)",
                        BillingInterval::Annually => "Annual (switch to Monthly)",
                    }
                })))
                .event(clone!(state => move |_evt: events::Click| {
                    let billing_interval = state.billing_interval.get_cloned();

                    match billing_interval {
                        BillingInterval::Monthly => state.billing_interval.set(BillingInterval::Annually),
                        BillingInterval::Annually => state.billing_interval.set(BillingInterval::Monthly),
                    }
                }))
            }))
            .child_signal(subscription_type_signal.map(clone!(state => move |subscription_type| {
                match subscription_type {
                    Some((subscription_type, plans)) => {
                        Some(match subscription_type {
                            SubscriptionType::Individual => state.clone().render_individual_plans(Rc::clone(&plans)),
                            SubscriptionType::School => state.clone().render_school_plans(Rc::clone(&plans)),
                        })
                    }
                    None => None, // TODO add loading state
                }
            })))
        })
    }

    fn render_payment_details(self: Rc<Self>, subscription: SelectedSubscription) -> Dom {
        let state = self;
        let user = get_user_cloned().unwrap_ji();

        let existing_payment_method = Mutable::new(user.payment_method.clone());

        html!("empty-fragment", {
            .child_signal(existing_payment_method.signal_cloned().map(clone!(state, existing_payment_method => move |payment_method| {
                let start_subscription_label = if user.subscription.is_some() { "Start subscription" } else { "Start trial" };
                let subscribe_url = format!("{}/subscribe/{}", SETTINGS.get().unwrap_ji().remote_target.pages_url(), subscription.plan_id);

                if let Some(payment_method) = payment_method {
                    Some(html!("empty-fragment", {
                        .child(html!("div", {
                            .text(&format!("{payment_method:?}"))
                        }))
                        .child(html!("button-rect", {
                            .text(start_subscription_label)
                            .prop("href", subscribe_url)
                            // TODO We could just subscribe on click here instead of navigating to another page
                        }))
                        .child(html!("button", {
                            .text("Use new payment method")
                            .event(clone!(existing_payment_method => move |_evt: events::Click| {
                                existing_payment_method.set(None);
                            }))
                        }))
                    }))
                } else {
                    let stripe_client_secret: Mutable<Option<String>> = Mutable::new(None);
                    Some(html!("empty-fragment", {
                        .future(clone!(stripe_client_secret => async move {
                            stripe_client_secret.set(Some(CreateSetupIntent::api_with_auth(CreateSetupIntentPath(), None)
                                .await
                                .unwrap_ji())); // TODO handle un-auth failure
                        }))
                        .child(html!("stripe-payment-details", {
                            .prop("publishableKey", "pk_test_cSUzMZSsUmmgzHRXPNEq5YOm") // TODO this needs to come from an env_var
                            .prop("redirectUrl", subscribe_url)
                            .prop("buttonLabel", start_subscription_label)
                            .prop_signal("clientSecret", stripe_client_secret.signal_cloned())
                        }))
                    }))
                }
            })))
        })
    }

    fn render_individual_plans(self: Rc<Self>, plans: Rc<SubscriptionPlansResponse>) -> Dom {
        let state = self;
        html!("empty-fragment", {
            .child_signal(state.billing_interval.signal_cloned().map(clone!(state => move |billing_interval| {
                let (basic, pro) = {
                    let (basic, pro) = match billing_interval {
                        BillingInterval::Monthly => (&plans.individual.basic_monthly, &plans.individual.pro_monthly),
                        BillingInterval::Annually => (&plans.individual.basic_annual, &plans.individual.pro_annual),
                    };

                    let basic = plans.plans.get(basic).unwrap_ji();
                    let pro = plans.plans.get(pro).unwrap_ji();

                    (basic, pro)
                };

                Some(html!("div", {
                    .children([
                        state.render_individual_plan(basic),
                        state.render_individual_plan(pro),
                    ])
                }))
            })))
        })
    }

    fn render_individual_plan(self: &Rc<Self>, plan: &SubscriptionPlanDetailsResponse) -> Dom {
        let state = self;
        html!("div", {
            .text(&format!(
                "Plan ID: {:?}, Amount in cents: {:?}, Trial period: {:?}",
                plan.plan_id,
                plan.amount_in_cents,
                plan.trial_period)
            )
            .apply(|dom| {
                if is_user_set() {
                    dom.child(html!("button", {
                        .text("Subscribe")
                        .event(clone!(state, plan => move |_evt: events::Click| {
                            state.subscribe_state.set(SubscribeState::PaymentMethod(SelectedSubscription::new(plan.clone())));
                        }))
                    }))
                } else {
                    dom.child(html!("button", {
                        .text("Login to subscribe")
                        .event(clone!(state => move |_evt: events::Click| {
                            // TODO login
                        }))
                    }))
                }
            })
        })
    }

    fn render_school_plans(self: Rc<Self>, plans: Rc<SubscriptionPlansResponse>) -> Dom {
        let state = self;
        html!("empty-fragment", {
            .child_signal(state.billing_interval.signal_cloned().map(clone!(state => move |billing_interval| {
                let (limited, unlimited) = match billing_interval {
                    BillingInterval::Monthly => {
                        get_school_plans(&plans.school.limited_monthly, plans.school.unlimited_monthly, &plans.plans)
                    }
                    BillingInterval::Annually => {
                        get_school_plans(&plans.school.limited_annual, plans.school.unlimited_annual, &plans.plans)
                    }
                };

                let limited = limited.iter().map(clone!(state => move |(account_limit, plan)| {
                        state.render_school_plan(Some(account_limit), plan)
                    })).collect::<Vec<_>>();

                Some(html!("div", {
                    .children(limited)
                    .child(state.render_school_plan(None, &unlimited))
                }))
            })))
        })
    }

    fn render_school_plan(self: &Rc<Self>, account_limit: Option<&AccountLimit>, plan: &SubscriptionPlanDetailsResponse) -> Dom {
        let state = self;
        html!("div", {
            .text(&format!(
                "Limit: {account_limit:?}, Plan ID: {:?}, Amount in cents: {:?}, Trial period: {:?}",
                plan.plan_id,
                plan.amount_in_cents,
                plan.trial_period)
            )
            .apply(|dom| {
                if is_user_set() {
                    dom.child(html!("button", {
                        .text("Subscribe")
                        .event(clone!(state, plan => move |_evt: events::Click| {
                            state.subscribe_state.set(SubscribeState::PaymentMethod(SelectedSubscription::new(plan.clone())));
                        }))
                    }))
                } else {
                    dom.child(html!("button", {
                        .text("Login to subscribe")
                        .event(clone!(state => move |_evt: events::Click| {
                            // TODO login
                        }))
                    }))
                }
            })
        })
    }
}

fn get_school_plans(
    limited: &HashMap<AccountLimit, PlanId>,
    unlimited: PlanId,
    plans: &HashMap<PlanId, SubscriptionPlanDetailsResponse>,
) -> (
    Vec<(AccountLimit, SubscriptionPlanDetailsResponse)>,
    SubscriptionPlanDetailsResponse,
) {
    let mut limited: Vec<_> = limited
        .iter()
        .map(|(account_limit, plan_id)| {
            (*account_limit, plans.get(plan_id).unwrap_ji().clone())
        })
        .collect();

    limited.sort_by_key(|(account_limit, _)| *account_limit);

    let unlimited = plans
        .get(&unlimited)
        .unwrap_ji()
        .clone();

    (limited, unlimited)
}
