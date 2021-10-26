use std::rc::Rc;
use super::state::SaySomething;
use utils::prelude::*;
use dominator::{Dom, html, clone};

impl SaySomething {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
        })
    }
}