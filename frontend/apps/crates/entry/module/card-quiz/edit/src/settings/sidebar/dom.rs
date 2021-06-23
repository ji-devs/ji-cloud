use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::state::*;
use super::state::*;

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    html!("div")
}

