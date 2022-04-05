use dominator::{html, Dom};

use crate::strings::register::complete::*;

pub struct CompletePage {}

impl CompletePage {
    pub fn render() -> Dom {
        html!("page-register-complete", {
            .child(
                html!("a", {
                    .property("slot", "button")
                    .attribute("href", "/")
                    .style("text-decoration", "none")
                    .child(
                        html!("button-rect", {
                            .property("color", "red")
                            .property("size", "small")
                            .text(STR_SUBMIT)
                        })
                    )
                })
            )
        })
    }
}
