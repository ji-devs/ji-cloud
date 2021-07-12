use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::state::*;

impl StickerOutline {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div")
    }
}
