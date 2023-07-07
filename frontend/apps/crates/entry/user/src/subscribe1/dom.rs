use super::stripe::Stripe;

use super::state::Subscribe1;
use dominator::{clone, html, with_node, DomBuilder};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::{component::Component, events, gap, icon};
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
                    .text("Try Jigzi school FREE for 14 days")
                    .text(" : ")
                    .text(state.plan_type.as_str())
                }))
                .child(html!("p", {
                    .class("list-item")
                    .child(icon!("fa-solid fa-check"))
                    .text("Get a 14 day trial, cancel any time.")
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
