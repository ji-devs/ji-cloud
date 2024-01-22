use awsm_web::loaders::fetch::fetch_url;
use components::{page_footer, page_header::PageHeader};
use const_format::formatcp;
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use gloo::utils::window;
use js_sys::Date;
use shared::domain::billing::{
    BillingInterval, PlanType, PLAN_SCHOOL_LEVEL_1_TEACHER_COUNT,
    PLAN_SCHOOL_LEVEL_2_TEACHER_COUNT, PLAN_SCHOOL_LEVEL_3_TEACHER_COUNT,
    PLAN_SCHOOL_LEVEL_4_TEACHER_COUNT,
};
use std::rc::Rc;
use utils::{
    constants::{INDIVIDUAL_FREE_TRIAL_DAYS, SCHOOL_FREE_TRIAL_DAYS},
    events, on_click_go_to_url,
    routes::{HomePlanRoute, HomePricingRoute, HomeRoute, Route, UserRoute},
    unwrap::UnwrapJiExt,
};

use super::{Pricing, Variables};

const PLAN_PRICE_MONTHLY_BASIC: u32 = 17_99;
const PLAN_PRICE_ANNUAL_BASIC: u32 = 180_00;
const PLAN_PRICE_MONTHLY_PRO: u32 = 29_99;
const PLAN_PRICE_ANNUAL_PRO: u32 = 300_00;
const PLAN_PRICE_MONTHLY_SCHOOL_1: u32 = 115_00;
const PLAN_PRICE_ANNUAL_SCHOOL_1: u32 = 1_250_00;
const PLAN_PRICE_MONTHLY_SCHOOL_2: u32 = 150_00;
const PLAN_PRICE_ANNUAL_SCHOOL_2: u32 = 1_500_00;
const PLAN_PRICE_MONTHLY_SCHOOL_3: u32 = 200_00;
const PLAN_PRICE_ANNUAL_SCHOOL_3: u32 = 2_000_00;
const PLAN_PRICE_MONTHLY_SCHOOL_4: u32 = 250_00;
const PLAN_PRICE_ANNUAL_SCHOOL_4: u32 = 2_500_00;
const PLAN_PRICE_MONTHLY_SCHOOL_UNLIMITED: u32 = 300_00;
const PLAN_PRICE_ANNUAL_SCHOOL_UNLIMITED: u32 = 3_000_00;

impl Pricing {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state: &Rc<Pricing> = self;
        html!("div", {
            .child(PageHeader::new(Default::default()).render())
            .future(clone!(state => async move {
                let str_config_url = format!("{}?{}", utils::path::config_cdn_url("pricing.json"), Date::now().to_string());
                let mut updated: Variables = fetch_url(&str_config_url)
                    .await
                    .unwrap_ji()
                    .json_from_str()
                    .await
                    .unwrap_ji();
                state.variables.replace_with(|current| {
                    if updated.bubble_color.is_empty() {
                        updated.bubble_color = current.bubble_color.clone();
                    }
                    if updated.bubble_title.is_empty() {
                        updated.bubble_title = current.bubble_title.clone();
                    }
                    if updated.bubble_message.is_empty() {
                        updated.bubble_message = current.bubble_message.clone();
                    }
                    updated
                });
            }))
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
            // TODO: enable once faq texts are updates
            .child(html!("pricing-faq", {
                .prop("id", "faq")
            }))
            .child(html!("button-rect", {
                .prop("kind", "outline")
                .prop("color", "blue")
                .style("position", "fixed")
                .style("right", "16px")
                .style("background-color", "#ffffff")
                .style("z-index", "10000")
                .style("bottom", "16px")
                .prop("href", "#faq")
                .text("Questions?")
            }))
            .child(page_footer::dom::render(None))
        })
    }

    fn render_individual(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let screen_size = Mutable::new(get_current_screen_size());
        vec![
            html!("pricing-toggle", {
                .prop("annual_label", "Get 2 months FREE!")
                .prop_signal("value", state.billing_interval.signal().map(|f| -> &str {(&f).into()}))
                .event(clone!(state => move |e: events::CustomString| {
                    let value: &str = &e.value();
                    state.billing_interval.set(value.try_into().unwrap_ji());
                }))
            }),
            html!("pricing-table", {
                .global_event(clone!(screen_size => move |_: events::Resize| {
                    screen_size.set(get_current_screen_size())
                }))
                .prop("kind", "individuals")
                .prop_signal("frequency", state.billing_interval.signal().map(|billing_interval| billing_interval.as_str()))
                .prop_signal("plan_price_basic", state.billing_interval.signal().map(|billing_interval| match billing_interval {
                    BillingInterval::Annually => PLAN_PRICE_ANNUAL_BASIC,
                    BillingInterval::Monthly => PLAN_PRICE_MONTHLY_BASIC,
                }))
                .prop_signal("plan_price_pro", state.billing_interval.signal().map(|billing_interval| match billing_interval {
                    BillingInterval::Annually => PLAN_PRICE_ANNUAL_PRO,
                    BillingInterval::Monthly => PLAN_PRICE_MONTHLY_PRO,
                }))
                .prop_signal("discount_percentage_basic", state.variables.signal_ref(|v| v.discount_percentage_basic))
                .prop_signal("discount_percentage_pro", state.variables.signal_ref(|v| v.discount_percentage_pro))
                .child(html!("pricing-message", {
                    .prop("slot", "pricing-message")
                    .prop_signal("color", state.variables.signal_ref(|v| v.bubble_color.clone()))
                    .prop_signal("title", state.variables.signal_ref(|v| v.bubble_title.clone()))
                    .prop_signal("message", state.variables.signal_ref(|v| v.bubble_message.clone()))
                }))
                // .child(html!("button-rect", {
                //     .prop("slot", "free-action")
                //     .prop("kind", "filled")
                //     .prop("color", "blue")
                //     .prop_signal("href", billing_interval.signal().map(|billing_interval| {
                //         let plan = match billing_interval {

                //         };
                //         Route::User(UserRoute::Subscribe(plan)).to_string()
                //     }))
                //     .text("Start 7-day trial")
                // }))
                .child(html!("button-rect", {
                    .prop("slot", "basic-action")
                    .prop("kind", "filled")
                    .prop("color", "blue")
                    .prop_signal("href", map_ref! {
                        let billing_interval = state.billing_interval.signal(),
                        let promo_code = state.basic_promo_code_signal() => {
                            let plan = match billing_interval {
                                BillingInterval::Annually => PlanType::IndividualBasicAnnually,
                                BillingInterval::Monthly => PlanType::IndividualBasicMonthly,
                            };
                            Route::User(UserRoute::Subscribe1(plan, promo_code.clone())).to_string()
                        }
                    })
                    .text_signal(screen_size.signal().map(|size| {
                        match size {
                            ScreenSize::Mobile => "Start trial".to_owned(),
                            ScreenSize::Desktop => formatcp!("Start {}-day trial", INDIVIDUAL_FREE_TRIAL_DAYS).to_owned(),
                        }
                    }))
                }))
                .child(html!("button-rect", {
                    .prop("slot", "pro-action")
                    .prop("kind", "filled")
                    .prop("color", "blue")
                    .prop_signal("href", map_ref! {
                        let billing_interval = state.billing_interval.signal(),
                        let promo_code = state.pro_promo_code_signal() => {
                            let plan = match billing_interval {
                                BillingInterval::Annually => PlanType::IndividualProAnnually,
                                BillingInterval::Monthly => PlanType::IndividualProMonthly,
                            };
                            Route::User(UserRoute::Subscribe1(plan, promo_code.clone())).to_string()
                        }
                    })
                    .text_signal(screen_size.signal().map(move|size| {
                        match size {
                            ScreenSize::Mobile => "Start trial".to_owned(),
                            ScreenSize::Desktop => formatcp!("Start {}-day trial", INDIVIDUAL_FREE_TRIAL_DAYS).to_owned(),
                        }
                    }))
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
                        .on_click_go_to_url!(Route::Home(HomeRoute::Plan(HomePlanRoute::Pro)))
                    }),
                ])
            }),
        ]
    }

    fn render_school(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let selected_index: Mutable<SchoolPlan> = Mutable::new(SchoolPlan::default());

        vec![
            html!("pricing-toggle", {
                .prop_signal("annual_label", selected_index.signal().map(|selected_index| {
                    match selected_index {
                        SchoolPlan::Level1 => "Get 1 month FREE!",
                        _ => "Get 2 months FREE!"
                    }
                }))
                .prop_signal("value", state.billing_interval.signal().map(|f| -> &str {(&f).into()}))
                .event(clone!(state => move |e: events::CustomString| {
                    let value: &str = &e.value();
                    state.billing_interval.set(value.try_into().unwrap_ji());
                }))
            }),
            html!("pricing-table", {
                .prop("kind", "schools")
                .child(html!("div", {
                    .prop("slot", "school-head")
                    .child(html!("pricing-message", {
                        .prop_signal("color", state.variables.signal_ref(|v| v.bubble_color.clone()))
                        .prop_signal("title", state.variables.signal_ref(|v| v.bubble_title.clone()))
                        .prop_signal("message", state.variables.signal_ref(|v| v.bubble_message.clone()))
                    }))
                    .child(html!("pricing-school-pricing", {
                        .prop_signal("billing_interval", state.billing_interval.signal().map(|billing_interval| billing_interval.display_name()))
                        .prop("school_level_1_max", PLAN_SCHOOL_LEVEL_1_TEACHER_COUNT)
                        .prop("school_level_2_max", PLAN_SCHOOL_LEVEL_2_TEACHER_COUNT)
                        .prop("school_level_3_max", PLAN_SCHOOL_LEVEL_3_TEACHER_COUNT)
                        .prop("school_level_4_max", PLAN_SCHOOL_LEVEL_4_TEACHER_COUNT)
                        .prop_signal("plan_price", map_ref! {
                            let selected_index = selected_index.signal(),
                            let billing_interval = state.billing_interval.signal() => move {
                                match billing_interval {
                                    BillingInterval::Annually => match selected_index {
                                        SchoolPlan::Level1 => PLAN_PRICE_ANNUAL_SCHOOL_1,
                                        SchoolPlan::Level2 => PLAN_PRICE_ANNUAL_SCHOOL_2,
                                        SchoolPlan::Level3 => PLAN_PRICE_ANNUAL_SCHOOL_3,
                                        SchoolPlan::Level4 => PLAN_PRICE_ANNUAL_SCHOOL_4,
                                        SchoolPlan::Unlimited => PLAN_PRICE_ANNUAL_SCHOOL_UNLIMITED,
                                    },
                                    BillingInterval::Monthly => match selected_index {
                                        SchoolPlan::Level1 => PLAN_PRICE_MONTHLY_SCHOOL_1,
                                        SchoolPlan::Level2 => PLAN_PRICE_MONTHLY_SCHOOL_2,
                                        SchoolPlan::Level3 => PLAN_PRICE_MONTHLY_SCHOOL_3,
                                        SchoolPlan::Level4 => PLAN_PRICE_MONTHLY_SCHOOL_4,
                                        SchoolPlan::Unlimited => PLAN_PRICE_MONTHLY_SCHOOL_UNLIMITED,
                                    },
                                }
                            }
                        })
                        .prop_signal("selectedIndex", selected_index.signal().map(|i| -> u8 {i.into()}))
                        .prop_signal("discount_percentage", state.variables.signal_ref(|v| v.discount_percentage_school))
                        .event(clone!(selected_index => move |e: events::CustomNumber| {
                            let index = e.number().unwrap_ji() as u8;
                            selected_index.set(index.try_into().unwrap_ji());
                        }))
                        .child(html!("button-rect", {
                            .prop("slot", "start-button")
                            .prop("kind", "filled")
                            .prop("color", "blue")
                            .text(formatcp!("Start {}-day trial", SCHOOL_FREE_TRIAL_DAYS))
                            .prop_signal("href", map_ref! {
                                let selected_index = selected_index.signal(),
                                let billing_interval = state.billing_interval.signal(),
                                let promo_code = state.school_promo_code_signal() => {
                                    let plan = selected_index.to_plan_type(*billing_interval);
                                    Route::User(UserRoute::SchoolStart(plan, promo_code.clone())).to_string()
                                }
                            })
                        }))
                    }))
                }))
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

    fn pro_promo_code_signal(self: &Rc<Self>) -> impl Signal<Item = Option<String>> {
        self.variables.signal_ref(|variables| {
            if variables.promo_code_pro.is_empty() {
                return None;
            }
            Some(variables.promo_code_pro.clone())
        })
    }

    fn basic_promo_code_signal(self: &Rc<Self>) -> impl Signal<Item = Option<String>> {
        self.variables.signal_ref(|variables| {
            if variables.promo_code_basic.is_empty() {
                return None;
            }
            Some(variables.promo_code_basic.clone())
        })
    }

    fn school_promo_code_signal(self: &Rc<Self>) -> impl Signal<Item = Option<String>> {
        self.variables.signal_ref(|variables| {
            if variables.promo_code_school.is_empty() {
                return None;
            }
            Some(variables.promo_code_school.clone())
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum SchoolPlan {
    Level1,
    #[default]
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

impl SchoolPlan {
    pub fn to_plan_type(self, billing_interval: BillingInterval) -> PlanType {
        match billing_interval {
            BillingInterval::Annually => match self {
                Self::Level1 => PlanType::SchoolLevel1Annually,
                Self::Level2 => PlanType::SchoolLevel2Annually,
                Self::Level3 => PlanType::SchoolLevel3Annually,
                Self::Level4 => PlanType::SchoolLevel4Annually,
                Self::Unlimited => PlanType::SchoolUnlimitedAnnually,
            },
            BillingInterval::Monthly => match self {
                Self::Level1 => PlanType::SchoolLevel1Monthly,
                Self::Level2 => PlanType::SchoolLevel2Monthly,
                Self::Level3 => PlanType::SchoolLevel3Monthly,
                Self::Level4 => PlanType::SchoolLevel4Monthly,
                Self::Unlimited => PlanType::SchoolUnlimitedMonthly,
            },
        }
    }
}

#[derive(Clone, Copy)]
enum ScreenSize {
    Mobile,
    Desktop,
}
impl From<f64> for ScreenSize {
    fn from(inner_width: f64) -> Self {
        if inner_width >= 1024.0 {
            Self::Desktop
        } else {
            Self::Mobile
        }
    }
}
fn get_current_screen_size() -> ScreenSize {
    let inner_width = window().inner_width().unwrap_ji().as_f64().unwrap_ji();
    inner_width.into()
}
