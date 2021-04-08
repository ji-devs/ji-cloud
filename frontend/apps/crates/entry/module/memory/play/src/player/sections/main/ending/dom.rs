use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::data::state::*;

pub struct EndingDom {
}

impl EndingDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("play-ending", {
        })
    }
}

