use dominator::{Dom, clone, html};
use std::rc::Rc;
use crate::data::state::State;

pub struct OverlayDom {
}

impl OverlayDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("div", { 
            .property("slot", "overlay")
            .text("overlay")
        })
    }
}
