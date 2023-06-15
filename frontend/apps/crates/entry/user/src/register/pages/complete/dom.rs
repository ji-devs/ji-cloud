use dominator::{html, Dom};

use crate::strings::register::complete::*;

pub struct CompletePage {}

impl CompletePage {
    pub fn render() -> Dom {
        html!("page-register-complete", {
            .child(
                html!("a", {
                    .prop("slot", "button")
                    .attr("href", "/")
                    .style("text-decoration", "none")
                    .child(
                        html!("button-rect", {
                            .prop("color", "red")
                            .prop("size", "regular")
                            .text(STR_SUBMIT)
                        })
                    )
                })
            )
        })
    }
}
