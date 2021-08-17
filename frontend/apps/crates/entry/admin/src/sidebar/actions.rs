use super::state::*;
use dominator::clone;
use std::rc::Rc;
use utils::{prelude::*, routes::Route};

impl SidebarItem {
    pub fn on_click(&self) {
        if !self.locked {
            let route:String = Route::Admin(self.route.clone()).into();
            dominator::routing::go_to_url(&route);
        }
    }
}
