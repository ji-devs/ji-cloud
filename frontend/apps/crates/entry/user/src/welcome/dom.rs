use std::rc::Rc;

use dominator::{html, Dom};

use crate::strings::register::complete::*;

use super::Welcome;

impl Welcome {
    pub fn render(self: &Rc<Self>) -> Dom {
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
