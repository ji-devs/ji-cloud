use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use utils::{events, routes::*};

const STR_SUBMIT:&'static str = "Go to JI home";

pub struct CompletePage {
}

impl CompletePage {
    pub fn render() -> Dom {

        html!("page-register-complete", {
            .child(
                html!("button-rect", {
                    .property("slot", "button")
                    .property("color", "red")
                    .property("size", "medium")
                    .text(STR_SUBMIT)
                    .event(|evt:events::Click| {
                        dominator::routing::go_to_url("/");
                    })
                })
            )
        })
    }
}

