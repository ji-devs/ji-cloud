use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};

pub fn render(state: Rc<Step2>) -> Dom {
    html!("div", {.text("step 2 here!") })
}
