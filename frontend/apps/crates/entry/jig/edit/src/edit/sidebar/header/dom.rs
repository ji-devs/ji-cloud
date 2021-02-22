use dominator::{html, Dom};
use shared::domain::jig::Jig;
use std::rc::Rc;

use super::super::state::State as SidebarState;
pub struct HeaderDom {
}

impl HeaderDom {
    pub fn render(sidebar_state: Rc<SidebarState>) -> Dom {
        html!("jig-edit-sidebar-header", {
            .property("slot", "header")
        })
    }
}
