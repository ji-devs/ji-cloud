use dominator::{clone, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use shared::domain::billing::PlanType;
use std::rc::Rc;
use utils::{
    events,
    routes::{HomePricingRoute, HomeRoute, Route, UserRoute},
    unwrap::UnwrapJiExt,
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
        let selected_index: Mutable<SchoolPlan> = Mutable::new(SchoolPlan::Level3);

        vec![
            html!("pricing-message", {}),
            html!("pricing-school-pricing", {
                .prop_signal("selectedIndex", selected_index.signal().map(|i| -> u8 {i.into()}))
                .event(clone!(selected_index => move |e: events::CustomNumber| {
                    let index = e.number().unwrap_ji() as u8;
                    selected_index.set(index.try_into().unwrap_ji());
                }))
                .child(html!("button-rect", {
                    .prop("slot", "start-button")
                    .prop("kind", "filled")
                    .prop("color", "blue")
                    .text("Start 7-day trial")
                    .prop_signal("href", selected_index.signal().map(|selected_index| {
                        Route::User(UserRoute::SchoolStart(selected_index.into())).to_string()
                    }))
                }))
            }),
            html!("pricing-table", {
                .prop("kind", "schools")
            }),
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SchoolPlan {
    Level1,
    Level2,
    Level3,
    Level4,
    Unlimited,
}

impl TryFrom<u8> for SchoolPlan {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Level1),
            1 => Ok(Self::Level2),
            2 => Ok(Self::Level3),
            3 => Ok(Self::Level4),
            4 => Ok(Self::Unlimited),
            _ => Err(()),
        }
    }
}

impl From<SchoolPlan> for u8 {
    fn from(value: SchoolPlan) -> Self {
        match value {
            SchoolPlan::Level1 => 0,
            SchoolPlan::Level2 => 1,
            SchoolPlan::Level3 => 2,
            SchoolPlan::Level4 => 3,
            SchoolPlan::Unlimited => 4,
        }
    }
}

impl From<SchoolPlan> for PlanType {
    fn from(value: SchoolPlan) -> Self {
        match value {
            SchoolPlan::Level1 => PlanType::SchoolLevel1,
            SchoolPlan::Level2 => PlanType::SchoolLevel2,
            SchoolPlan::Level3 => PlanType::SchoolLevel3,
            SchoolPlan::Level4 => PlanType::SchoolLevel4,
            SchoolPlan::Unlimited => PlanType::SchoolUnlimited,
        }
    }
}
