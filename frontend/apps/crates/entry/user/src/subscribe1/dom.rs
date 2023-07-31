use super::stripe::Stripe;

use super::state::Subscribe1;
use dominator::{clone, html, with_node, DomBuilder};
use futures_signals::signal::SignalExt;
use shared::domain::billing::PlanType;
use std::rc::Rc;
use utils::{
    component::Component,
    constants::{INDIVIDUAL_FREE_TRIAL_DAYS, SCHOOL_FREE_TRIAL_DAYS},
    events, gap, icon,
};
use web_sys::{HtmlElement, ShadowRoot};

impl Component<Subscribe1> for Rc<Subscribe1> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn apply_on_host(&self, host: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        let state = self;
        host.child_signal(state.stripe_client_secret.signal_cloned().map(clone!(state => move |secret| {
            secret.map(|secret| {
                html!("div" => HtmlElement, {
                    .with_node!(elem => {
                        .apply(clone!(state => move|dom| {
                            state.loader.load(clone!(state => async move {
                                *state.stripe.borrow_mut() = Some(Stripe::new(elem, secret).await);
                            }));
                            dom
                        }))
                    })
                })
            })
        })))
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.start_intent();

        dom.child(html!("auth-page", {
            .prop("img", "entry/user/side/main.webp")
            .child(html!("main", {
                .child(html!("h1", {
                    .text("Try Jigzi ")
                    .text(state.plan_type.display_name())
                    .text(" FREE for ")
                    .text(&match state.plan_type {
                        PlanType::IndividualBasicMonthly | PlanType::IndividualBasicAnnually | PlanType::IndividualProMonthly | PlanType::IndividualProAnnually => INDIVIDUAL_FREE_TRIAL_DAYS,
                        PlanType::SchoolLevel1 | PlanType::SchoolLevel2 | PlanType::SchoolLevel3 | PlanType::SchoolLevel4 | PlanType::SchoolUnlimited => SCHOOL_FREE_TRIAL_DAYS,
                    }.to_string())
                    .text(" days")
                }))
                .child(html!("p", {
                    .class("list-item")
                    .child(icon!("fa-solid fa-check"))
                    .text("Get a ")
                    .text(&match state.plan_type {
                        PlanType::IndividualBasicMonthly | PlanType::IndividualBasicAnnually | PlanType::IndividualProMonthly | PlanType::IndividualProAnnually => INDIVIDUAL_FREE_TRIAL_DAYS,
                        PlanType::SchoolLevel1 | PlanType::SchoolLevel2 | PlanType::SchoolLevel3 | PlanType::SchoolLevel4 | PlanType::SchoolUnlimited => SCHOOL_FREE_TRIAL_DAYS,
                    }.to_string())
                    .text(" day trial, cancel any time.")
                }))
                .child(html!("p", {
                    .class("list-item")
                    .child(icon!("fa-solid fa-check"))
                    .text("Get a reminder 24 hours before your trial ends.")
                }))
                .child(gap!(30))
                .child(html!("h2", {
                    .text("Payment method")
                }))
                .child(html!("hr"))
                .child(html!("h2", {
                    .text("Request other payment method")
                }))
                .child(html!("slot"))
                .child(gap!(48))
                .child_signal(state.stripe_client_secret.signal_cloned().map(
                    clone!(state => move |secret| {
                        secret.map(|_| {
                            html!("button-rect", {
                                .prop("size", "large")
                                .text("Start free trial")
                                .event(clone!(state => move |_: events::Click| {
                                    state.submit();
                                }))
                            })
                        })
                    }),
                ))
            }))
        }))
    }
}
