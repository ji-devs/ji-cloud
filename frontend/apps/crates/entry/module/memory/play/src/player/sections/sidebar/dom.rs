use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::data::state::*;

pub struct SidebarDom {
}

impl SidebarDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("play-sidebar", {
            .property("slot", "sidebar")
        })
    }
}

