use dominator::{clone, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use shared::domain::billing::PlanType;
use std::rc::Rc;
use utils::{
    events, on_click_go_to_url,
    routes::{HomePlanRoute, HomePricingRoute, HomeRoute, Route, UserRoute},
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
                .style("position", "fixed")
                .style("right", "16px")
                .style("background-color", "#ffffff")
                .style("z-index", "10000")
                .style("bottom", "16px")
                .text("Questions?")
            }))
        })
    }

    fn render_individual(self: &Rc<Self>) -> Vec<Dom> {
        let frequency = Mutable::new(Frequency::Monthly);
        vec![
            html!("pricing-toggle", {
                .prop_signal("value", frequency.signal().map(|f| -> &str {f.into()}))
                .event(clone!(frequency => move |e: events::CustomString| {
                    let value: &str = &e.value();
                    frequency.set(value.try_into().unwrap_ji());
                }))
            }),
            html!("pricing-table", {
                .prop("kind", "individuals")
                .prop_signal("frequency", frequency.signal().map(|frequency| match frequency {
                    Frequency::Annually => "Annually",
                    Frequency::Monthly => "Monthly",
                }))
                .child(html!("pricing-message", {
                    .prop("slot", "pricing-message")
                }))
                // .child(html!("button-rect", {
                //     .prop("slot", "free-action")
                //     .prop("kind", "filled")
                //     .prop("color", "blue")
                //     .prop_signal("href", frequency.signal().map(|frequency| {
                //         let plan = match frequency {

                //         };
                //         Route::User(UserRoute::Subscribe(plan)).to_string()
                //     }))
                //     .text("Start 7-day trial")
                // }))
                .child(html!("button-rect", {
                    .prop("slot", "basic-action")
                    .prop("kind", "filled")
                    .prop("color", "blue")
                    .prop_signal("href", frequency.signal().map(|frequency| {
                        let plan = match frequency {
                            Frequency::Annually => PlanType::IndividualBasicAnnually,
                            Frequency::Monthly => PlanType::IndividualBasicMonthly,
                        };
                        Route::User(UserRoute::Subscribe1(plan)).to_string()
                    }))
                    .text("Start 7-day trial")
                }))
                .child(html!("button-rect", {
                    .prop("slot", "pro-action")
                    .prop("kind", "filled")
                    .prop("color", "blue")
                    .prop_signal("href", frequency.signal().map(|frequency| {
                        let plan = match frequency {
                            Frequency::Annually => PlanType::IndividualProAnnually,
                            Frequency::Monthly => PlanType::IndividualProMonthly,
                        };
                        Route::User(UserRoute::Subscribe1(plan)).to_string()
                    }))
                    .text("Start 7-day trial")
                }))
                .children(&mut [
                    // html!("button-rect", {
                    //     .prop("slot", "learn-more-free")
                    //     .prop("kind", "text")
                    //     .prop("color", "blue")
                    //     .text("Learn more")
                    //     .on_click_go_to_url!(Route::Home(HomeRoute::Plan(HomePlanRoute::Free)))
                    // }),
                    html!("button-rect", {
                        .prop("slot", "learn-more-basic")
                        .prop("kind", "text")
                        .prop("color", "blue")
                        .text("Learn more")
                        .on_click_go_to_url!(Route::Home(HomeRoute::Plan(HomePlanRoute::Basic)))
                    }),
                    html!("button-rect", {
                        .prop("slot", "learn-more-pro")
                        .prop("kind", "text")
                        .prop("color", "blue")
                        .text("Learn more")
                        .on_click_go_to_url!(Route::Home(HomeRoute::Plan(HomePlanRoute::Basic)))
                    }),
                ])
            }),
        ]
    }

    fn render_school(self: &Rc<Self>) -> Vec<Dom> {
        let selected_index: Mutable<SchoolPlan> = Mutable::new(SchoolPlan::Level3);

        vec![
            html!("div", {
                .style("display", "grid")
                .style("grid-template-columns", "auto auto")
                .style("align-items", "center")
                .style("justify-content", "space-around")
                .style("align-items", "end")
                .child(html!("pricing-message", {}))
                .child(html!("pricing-school-pricing", {
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
                }))
            }),
            html!("pricing-table", {
                .prop("kind", "schools")
                .child(html!("button-rect", {
                    .prop("slot", "learn-more-school")
                    .prop("kind", "text")
                    .prop("color", "blue")
                    .text("Learn more")
                    .on_click_go_to_url!(Route::Home(HomeRoute::Plan(HomePlanRoute::School)))
                }))
            }),
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Frequency {
    Annually,
    Monthly,
}

impl TryFrom<&str> for Frequency {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "annually" => Ok(Self::Annually),
            "monthly" => Ok(Self::Monthly),
            _ => Err(()),
        }
    }
}

impl From<Frequency> for &str {
    fn from(value: Frequency) -> Self {
        match value {
            Frequency::Annually => "annually",
            Frequency::Monthly => "monthly",
        }
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
