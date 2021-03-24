use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::data::state::*;

pub struct HeaderDom {
}

impl HeaderDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("play-header", {
            .property("slot", "header")
        })
    }
}

