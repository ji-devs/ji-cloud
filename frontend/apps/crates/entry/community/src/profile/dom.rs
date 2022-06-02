use std::rc::Rc;

use dominator::{html, Dom};

use super::CommunityProfile;

impl CommunityProfile {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .text("profile")
        })
    }
}
