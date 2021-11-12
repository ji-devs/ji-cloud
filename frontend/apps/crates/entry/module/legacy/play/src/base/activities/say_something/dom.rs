use super::state::SaySomething;
use std::rc::Rc;

use crate::base::styles;
use dominator::{html, Dom};

impl SaySomething {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .class(&*styles::FULL_STAGE)
        })
    }
}
