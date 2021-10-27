use std::rc::Rc;
use super::state::SaySomething;
use utils::prelude::*;
use dominator::{Dom, html, clone};
use crate::base::styles;

impl SaySomething {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .class(&*styles::FULL_STAGE)
        })
    }
}