use dominator::{Dom, clone, html};
use std::rc::Rc;
use crate::data::state::State;
use crate::steps::nav::dom::StepsNavDom;
use utils::prelude::*;

pub struct HeaderDom {
}

impl HeaderDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("module-preview-header", {
            .property("slot", "header")
            .child(StepsNavDom::render(state.clone()))
            .child(html!("button-rect", {
                .property("slot", "btn")
                .property("size", "small")
                .property("iconAfter", "arrow")
                .text(crate::strings::STR_DONE)
                .event(clone!(state => move |evt:events::Click| {
                }))
            }))
        })
    }
}

