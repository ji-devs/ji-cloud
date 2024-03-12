use std::rc::Rc;

use futures_signals::signal::Mutable;
use serde::{Deserialize, Serialize};
use shared::domain::billing::BillingInterval;
use utils::routes::HomePricingRoute;

pub struct Pricing {
    pub(super) route: Mutable<HomePricingRoute>,
    pub(super) variables: Mutable<Variables>,
    pub(super) billing_interval: Mutable<BillingInterval>,
}

impl Pricing {
    pub fn new(route: HomePricingRoute) -> Rc<Self> {
        Rc::new(Self {
            route: Mutable::new(route),
            variables: Default::default(),
            billing_interval: Mutable::new(BillingInterval::Annually),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Variables {
    pub discount_percentage_pro: u32,
    pub discount_percentage_basic: u32,
    pub discount_percentage_school: u32,
    pub promo_code_pro: String,
    pub promo_code_basic: String,
    pub promo_code_school: String,
    pub bubble_title: String,
    pub bubble_message: String,
    pub bubble_color: String,
}
impl Default for Variables {
    fn default() -> Self {
        Self {
            discount_percentage_pro: Default::default(),
            discount_percentage_basic: Default::default(),
            discount_percentage_school: Default::default(),
            promo_code_pro: Default::default(),
            promo_code_basic: Default::default(),
            promo_code_school: Default::default(),
            bubble_color: String::from("var(--light-orange-3)"),
            bubble_title: String::from("Sign up now!"),
            bubble_message: String::from(
                "Thousands of educators around the world use Jigzi for their lessons.",
            ),
        }
    }
}
