use dominator::{Dom, clone, html};
use std::rc::Rc;
use crate::data::state::State;

pub struct MainDom {
}

impl MainDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("div", { 
            .property("slot", "main")
            .text("main")
        })
    }
}
