use dominator::{Dom, clone, html};
use std::rc::Rc;
use crate::data::state::State;

pub struct FooterDom {
}

impl FooterDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("div", { 
            .property("slot", "footer")
            .text("footer")
        })
    }
}
