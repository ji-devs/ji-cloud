use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use crate::strings::register::complete::*;

pub struct CompletePage {
}

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
                            .property("size", "medium")
                            .text(STR_SUBMIT)
                        })
                    )
                })
            )
        })
    }
}

