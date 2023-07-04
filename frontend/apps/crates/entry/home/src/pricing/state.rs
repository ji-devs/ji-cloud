use std::rc::Rc;

use futures_signals::signal::Mutable;
use utils::routes::HomePricingRoute;

pub struct Pricing {
    pub(super) route: Mutable<HomePricingRoute>,
}

impl Pricing {
    pub fn new(route: HomePricingRoute) -> Rc<Self> {
        Rc::new(Self {
            route: Mutable::new(route),
        })
    }
}
