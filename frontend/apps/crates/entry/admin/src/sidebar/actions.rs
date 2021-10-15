use super::state::*;

use utils::routes::Route;

impl SidebarItem {
    pub fn on_click(&self) {
        if !self.locked {
            let route: String = Route::Admin(self.route.clone()).into();
            dominator::routing::go_to_url(&route);
        }
    }
}
