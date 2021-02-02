use std::rc::Rc;
use dominator::{html, Dom};
use utils::{events, routes::*};


const STR_BUTTON:&'static str = "Go to JI Home";

pub struct RegisterCompletePage {
}

impl RegisterCompletePage {
    pub fn render() -> Dom {
        html!("page-register-complete", {
            .child(
                html!("button-rect", {
                    .property("slot", "button")
                    .property("color", "red")
                    .property("size", "medium")
                    .text(STR_BUTTON)
                    .event(|evt:events::Click| {
                        //actions::signin_email(state.clone())
                    })
                })
            )
        })
    }
}
