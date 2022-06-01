use std::rc::Rc;

use dominator::{Dom, html};

use super::CommunityBadges;

impl CommunityBadges {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .text("badges")
        })
    }
}
