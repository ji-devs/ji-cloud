use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::signal::SignalExt;
use utils::events;

use crate::student_code::actions::submit_code;

use super::state::State;

const STR_TRY_AGAIN: &'static str = "Try again";
const STR_HELP: &'static str = "Ask for help";

pub fn render(state: Rc<State>) -> Dom {
    html!("home-student-code", {
        .child(html!("home-student-code-input", {
            .property("slot", "input")
            .property_signal("error", state.error.signal())
            .event(clone!(state => move |evt: events::CustomInput| {
                submit_code(Rc::clone(&state), evt.value());
            }))
        }))
        .child(html!("home-student-code-jigzi", {
            .property("slot", "jigzi")
            .property_signal("mode", state.error.signal().map(|error| {
                match error {
                    true => "try-again",
                    false => "default",
                }
            }))
            .child(html!("button", {
                .property("slot", "try-again")
                .text(STR_TRY_AGAIN)
            }))
            .child(html!("button", {
                .property("slot", "help")
                .text(STR_HELP)
            }))
        }))
    })
}
