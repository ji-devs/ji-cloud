use std::rc::Rc;

use dominator::{Dom, html};

use super::CommunityProfile;

impl CommunityProfile {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .text("profile")
        })
    }
}
