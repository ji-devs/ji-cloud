use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};

pub fn render(state: Rc<Step1>) -> Dom {
    html!("div", {.text("step 1 here!") })
}
