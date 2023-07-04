use crate::subscribe::stripe::Stripe;

use super::state::Subscribe;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::{
    events,
    prelude::{get_school_id, SETTINGS},
    routes::{Route, UserRoute},
    unwrap::UnwrapJiExt,
};
use web_sys::HtmlElement;

impl Subscribe {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        state.start_intent();

        html!("div", {
            .child(html!("p", {
                .text("Plan: ")
                .text(state.plan_type.as_str())
            }))
            .children_signal_vec(state.stripe_client_secret.signal_cloned().map(clone!(state => move |secret| {
                secret.map(clone!(state => move |secret| {
                    vec![
                        html!("div" => HtmlElement, {
                            .with_node!(elem => {
                                .apply(clone!(state => move|dom| {
                                    state.loader.load(clone!(state => async move {
                                        *state.stripe.borrow_mut() = Some(Stripe::new(elem, secret).await);
                                    }));
                                    dom
                                }))
                            })
                        }),
                        html!("button-rect", {
                            .text("Submit")
                            .event(clone!(state => move |_: events::Click| {
                                let stripe = state.stripe.take();
                                state.loader.load(async move {
                                    stripe.unwrap_ji().submit(&get_next_page_url()).await;
                                });
                            }))
                        }),
                    ]
                })).unwrap_or_default()
            })).to_signal_vec())
        })
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
