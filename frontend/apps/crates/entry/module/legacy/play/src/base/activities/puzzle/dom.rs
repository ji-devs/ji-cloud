use super::state::*;
use std::rc::Rc;

use crate::base::styles;
use dominator::{html, Dom};

impl Puzzle {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .class(&*styles::FULL_STAGE)
        })
    }
}
