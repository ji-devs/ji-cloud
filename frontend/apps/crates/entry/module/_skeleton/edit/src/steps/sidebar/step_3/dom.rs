use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};

pub fn render(state: Rc<Step3>) -> Dom {
    html!("div", {.text("step 3 here!") })
}
