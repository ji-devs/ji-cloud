use std::rc::Rc;

use utils::routes::HomePlanRoute;

pub struct Plan {
    pub(super) route: HomePlanRoute,
}

impl Plan {
    pub fn new(route: HomePlanRoute) -> Rc<Self> {
        Rc::new(Self { route })
    }
}
