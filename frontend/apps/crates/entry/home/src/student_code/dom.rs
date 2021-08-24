use std::rc::Rc;

use components::player_popup::{PlayerPopup, PreviewPopupCallbacks};
use dominator::{Dom, clone, html};
use futures_signals::signal::SignalExt;
use shared::domain::jig::JigPlayerSettings;
use utils::events;

use crate::student_code::actions::submit_code;

use super::state::State;

const STR_TRY_AGAIN: &'static str = "Try again";
const STR_HELP: &'static str = "Ask for help";

pub fn render(state: Rc<State>) -> Dom {
    html!("empty-fragment", {
        .child(html!("home-student-code", {
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
        }))
        .child_signal(state.play_jig.signal_cloned().map(clone!(state => move|play_jig| {
            play_jig.map(|(jig_id, settings)| {
                let close = clone!(state => move || {
                    state.play_jig.set(None);
                });
                PlayerPopup::render(
                    Rc::new(PlayerPopup::new(jig_id, settings, PreviewPopupCallbacks::new(close))),
                    None
                )
            })
        })))
    })
}
