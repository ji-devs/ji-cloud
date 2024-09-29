use super::{check_popup::CheckPopup, stripe::Stripe};

use super::state::Subscribe1;
use components::confirm;
use dominator::{clone, html, with_node, DomBuilder};
use futures_signals::signal::SignalExt;
use shared::domain::billing::{INDIVIDUAL_TRIAL_PERIOD, SCHOOL_TRIAL_PERIOD};
use std::rc::Rc;
use utils::prelude::get_user_cloned;
use utils::routes::{Route, UserRoute};
use utils::{component::Component, dialog, events, gap, icon};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, HtmlInputElement, ShadowRoot};

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
                    .text(&match state.plan_type.is_individual_plan() {
                        true => INDIVIDUAL_TRIAL_PERIOD,
                        false => SCHOOL_TRIAL_PERIOD,
                    }.to_string())
                    .text(" days")
                }))
                .child(html!("p", {
                    .class("list-item")
                    .child(icon!("fa-solid fa-check"))
                    .text("Get a ")
                    .text(&match state.plan_type.is_individual_plan() {
                        true => INDIVIDUAL_TRIAL_PERIOD,
                        false => SCHOOL_TRIAL_PERIOD,
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
                .child(html!("slot"))
                .child(html!("div", {
                    .class("promo")
                    .child(html!("label", {
                        .text("Promo code")
                    }))
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .prop_signal("value", state.promo.signal_cloned().map(|promo| promo.unwrap_or_default()))
                            .event(clone!(state => move |_: events::Input| {
                                let v = elem.value();
                                let v = match v.trim().is_empty() {
                                    true => None,
                                    false => Some(v),
                                };
                                state.promo.set(v);
                            }))
                        })
                    }))
                }))
                .child(gap!(48))
                .child_signal(state.stripe_client_secret.signal_cloned().map(
                    clone!(state => move |secret| {
                        secret.map(|_| {
                            html!("div", {
                                .class("actions")
                                .apply_if(state.plan_type.is_school_plan(), |dom| {
                                    dom
                                    // .child(html!("button-rect", {
                                    //     .prop("kind", "text")
                                    //     .prop("color", "red")
                                    //     .text("Pay another way")
                                    //     .event(clone!(state => move |_: events::Click| {
                                    //         state.pay_with_check.set(true);
                                    //         state.plan_type;
                                    //     }))
                                    // }))
                                    .child(html!("button-rect", {
                                        .prop("kind", "text")
                                        .prop("color", "red")
                                        .text("Add later")
                                        .event(clone!(state => move |_: events::Click| {
                                            Route::User(UserRoute::Subscribe2(state.plan_type, None, state.promo.get_cloned())).go_to();
                                        }))
                                    }))
                                })
                                .child(html!("button-rect", {
                                    .prop("kind", "filled")
                                    .prop("color", "red")
                                    .prop("size", "large")
                                    .text("Start free trial")
                                    .event(clone!(state => move |_: events::Click| {
                                        spawn_local(clone!(state => async move {
                                            let has_previous_subscription = get_user_cloned()
                                                .and_then(|user| user.account_summary)
                                                .and_then(|summary| summary.subscription_status)
                                                .is_some();

                                            if !has_previous_subscription {
                                                state.submit();
                                            } else {
                                                let confirmed = confirm::Confirm {
                                                    title: "Welcome back to Jigzi!".to_string(),
                                                    message: "The free trial offer has already been redeemed for this account. By clicking 'Confirm' your subscription will begin and payment will be taken immediately.".to_string(),
                                                    confirm_text: "Confirm".to_string(),
                                                    cancel_text: "Cancel".to_string()
                                                }.confirm().await;
                                                if confirmed {
                                                    state.submit();
                                                }
                                            }

                                        }));
                                    }))
                                }))
                            })
                        })
                    }),
                ))
                .child_signal(state.pay_with_check.signal_cloned().map(clone!(state => move |pay_with_check| {
                    pay_with_check.then(|| {
                        dialog!{
                            .child(CheckPopup::new(&state).render())
                        }
                    })
                })))
            }))
        }))
    }
}
