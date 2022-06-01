use std::rc::Rc;

use dominator::{Dom, html};

use super::CommunityMembers;

impl CommunityMembers {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .text("members")
        })
    }
}
