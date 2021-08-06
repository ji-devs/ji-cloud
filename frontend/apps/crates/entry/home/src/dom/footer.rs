use dominator::{html, Dom};
use std::rc::Rc;

use super::super::state::State;

pub fn render(_state: Rc<State>) -> Dom {
    html!("home-footer", {
        .text("footer")
    })
}
