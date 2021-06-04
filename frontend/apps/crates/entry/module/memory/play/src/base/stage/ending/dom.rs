use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::base::state::*;

pub fn render(state: Rc<Base>) -> Dom {
    html!("play-ending", {
    })
}

