use dominator::{html, Dom};
use super::state::*;
use std::rc::Rc;

pub fn render(state: Rc<Ending>) -> Dom {
    html!("h1", {
        .text("Deck Finished!")
    })
}
