use dominator::{html, Dom};
use super::state::*;
use std::rc::Rc;

pub fn render(state: Rc<Ending>) -> Dom {
    html!("div", {
        .property("slot", "main")
        .style("position", "absolute")
        .style("color", "red")
        .style("font-size", "30rem")
        .text("Deck Finished!")
    })
}
