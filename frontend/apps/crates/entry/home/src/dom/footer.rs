use std::rc::Rc;
use dominator::{html, Dom};

use super::super::state::State;

pub fn render(_state: Rc<State>) -> Dom {
    html!("home-footer", {
        .text("footer")
    })
}
