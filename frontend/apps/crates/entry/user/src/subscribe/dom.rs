use crate::subscribe::stripe::Stripe;

use super::state::Subscribe;
use dominator::{clone, html, with_node, DomBuilder};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::{
    component::Component,
    events, gap, icon,
    prelude::{get_school_id, SETTINGS},
    routes::{Route, UserRoute},
    unwrap::UnwrapJiExt,
};
use web_sys::{HtmlElement, ShadowRoot};

impl Component<Subscribe> for Rc<Subscribe> {
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

        dom.child(html!("h1", {
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
                            let stripe = state.stripe.take();
                            state.loader.load(async move {
                                stripe.unwrap_ji().submit(&get_next_page_url()).await;
                            });
                        }))
                    })
                })
            }),
        ))
    }
}

fn get_next_page_url() -> String {
    let route = match get_school_id() {
        Some(_) => Route::User(UserRoute::SchoolEnd),
        None => Route::User(UserRoute::Welcome),
    };
    format!(
        "{}{}",
        SETTINGS.get().unwrap_ji().remote_target.pages_url(),
        route.to_string()
    )
}
