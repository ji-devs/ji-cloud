use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::billing::PlanType;
use std::rc::Rc;
use utils::{
    events,
    routes::{HomePricingRoute, HomeRoute, Route, UserRoute},
};

use super::Pricing;

impl Pricing {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .child(html!("pricing-banner", {
                .child(html!("button", {
                    .prop("slot", "tab")
                    .text("Individual")
                    .class_signal("active", state.route.signal().map(|route| {
                        route == HomePricingRoute::Individual
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        state.route.set(HomePricingRoute::Individual);
                        Route::Home(HomeRoute::Pricing(HomePricingRoute::Individual)).push_state();
                    }))
                }))
                .child(html!("button", {
                    .prop("slot", "tab")
                    .text("School")
                    .class_signal("active", state.route.signal().map(|route| {
                        route == HomePricingRoute::Schools
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        state.route.set(HomePricingRoute::Schools);
                        Route::Home(HomeRoute::Pricing(HomePricingRoute::Schools)).push_state();
                    }))
                }))
            }))
            .children_signal_vec(state.route.signal().map(clone!(state => move |route| {
                match route {
                    HomePricingRoute::Individual => state.render_individual(),
                    HomePricingRoute::Schools => state.render_school(),
                }
            })).to_signal_vec())
            .child(html!("pricing-faq", {

            }))
            .child(html!("button-rect", {
                .prop("kind", "outline")
                .prop("color", "blue")
                .text("Questions?")
            }))
        })
    }

    fn render_individual(self: &Rc<Self>) -> Vec<Dom> {
        vec![
            html!("button-rect", {
                .prop("href", Route::User(UserRoute::Subscribe(PlanType::IndividualBasicMonthly)).to_string())
                .text("Basic Monthly")
            }),
            html!("button-rect", {
                .prop("href", Route::User(UserRoute::Subscribe(PlanType::IndividualBasicAnnually)).to_string())
                .text("Basic Annually")
            }),
            html!("button-rect", {
                .prop("href", Route::User(UserRoute::Subscribe(PlanType::IndividualProMonthly)).to_string())
                .text("Pro Monthly")
            }),
            html!("button-rect", {
                .prop("href", Route::User(UserRoute::Subscribe(PlanType::IndividualProAnnually)).to_string())
                .text("Pro Annually")
            }),
            html!("pricing-toggle", {}),
            html!("pricing-table", {
                .prop("kind", "individuals")
            }),
        ]
    }

    fn render_school(self: &Rc<Self>) -> Vec<Dom> {
        vec![
            html!("button-rect", {
                .prop("href", Route::User(UserRoute::SchoolStart(PlanType::SchoolLevel1)).to_string())
                .text("Level 1")
            }),
            html!("button-rect", {
                .prop("href", Route::User(UserRoute::SchoolStart(PlanType::SchoolLevel2)).to_string())
                .text("Level 2")
            }),
            html!("button-rect", {
                .prop("href", Route::User(UserRoute::SchoolStart(PlanType::SchoolLevel3)).to_string())
                .text("Level 3")
            }),
            html!("button-rect", {
                .prop("href", Route::User(UserRoute::SchoolStart(PlanType::SchoolLevel4)).to_string())
                .text("Level 4")
            }),
            html!("button-rect", {
                .prop("href", Route::User(UserRoute::SchoolStart(PlanType::SchoolUnlimited)).to_string())
                .text("Unlimited")
            }),
            html!("pricing-message", {}),
            html!("pricing-school-pricing", {}),
            html!("pricing-table", {
                .prop("kind", "schools")
            }),
        ]
    }
}
