use std::rc::Rc;

use super::Schools;
use utils::routes::AdminSchoolsRoute;
use utils::routes::{AdminRoute, Route};

impl Schools {
    pub fn navigate_to(self: &Rc<Self>, route: AdminSchoolsRoute) {
        self.route.set(route.clone());
        Route::Admin(AdminRoute::Schools(route)).push_state();
    }
}
