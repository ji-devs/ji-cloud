use dominator::{Dom, clone, html};
use std::rc::Rc;
use crate::data::state::State;

pub struct HeaderDom {
}

impl HeaderDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("div", { 
            .property("slot", "header")
            .text("header")
        })
    }
}
