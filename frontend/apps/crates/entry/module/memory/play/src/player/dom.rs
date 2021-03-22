use dominator::{html, clone, Dom};
use crate::data::state::*;
use std::rc::Rc;

pub struct PlayerDom {}

impl PlayerDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("div")
    }
}

