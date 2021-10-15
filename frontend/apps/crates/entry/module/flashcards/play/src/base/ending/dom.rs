use super::state::*;
use dominator::{html, Dom};
use std::rc::Rc;

pub fn render(_state: Rc<Ending>) -> Dom {
    html!("div", {
        .property("slot", "main")
        .style("position", "absolute")
        .style("color", "red")
        .style("font-size", "30rem")
        .text("Deck Finished!")
    })
}
