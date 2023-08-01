use super::state::CheckPopup;
use dominator::{clone, html, DomBuilder};
use std::rc::Rc;
use utils::{
    component::Component,
    events, gap,
    routes::{Route, UserRoute},
};
use web_sys::ShadowRoot;

impl Component<CheckPopup> for Rc<CheckPopup> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom.child(html!("main", {
            .children(&mut [
                html!("fa-button", {
                    .prop("icon", "fa-light fa-xmark")
                    .event(clone!(state => move |_: events::Click| {
                        state.subscribe_1_state.pay_with_check.set(false);
                    }))
                }),
                html!("img-ui", {
                    .prop("path", "user/pay-with-check.webp")
                }),
                gap!(34),
                html!("h2", {
                    .text("Request other payment method")
                }),
                gap!(8),
                html!("h3", {
                    .text("Your trial will start now, We will contact you soon to set up the payment.")
                }),
                gap!(66),
                html!("div", {
                    .class("actions")
                    .child(html!("button-rect", {
                        .prop("kind", "text")
                        .prop("color", "blue")
                        .text("Cancel")
                        .event(clone!(state => move |_: events::Click| {
                            state.subscribe_1_state.pay_with_check.set(false);
                        }))
                    }))
                    .child(html!("button-rect", {
                        .prop("kind", "filled")
                        .prop("color", "blue")
                        .text("Start free trial")
                        .event(clone!(state => move |_: events::Click| {
                            Route::User(UserRoute::Subscribe2(state.subscribe_1_state.plan_type, None, state.subscribe_1_state.promo.clone())).go_to();
                        }))
                    }))
                })
            ])
        }))
    }
}
