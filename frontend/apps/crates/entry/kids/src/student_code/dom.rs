use std::rc::Rc;

use components::player_popup::{PlayerPopup, PreviewPopupCallbacks};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use utils::{events, jig::JigPlayerOptions};

use crate::student_code::actions::submit_code;

use super::state::State;

const STR_TRY_AGAIN: &'static str = "Try again";
const STR_HELP: &'static str = "Ask for help";

pub fn render(state: Rc<State>, code: Option<String>) -> Dom {
    if let Some(code) = code {
        submit_code(Rc::clone(&state), code);
    };

    html!("empty-fragment", {
        .child(html!("kids-student-code", {
            .child(html!("kids-student-code-input", {
                .property("slot", "input")
                .property_signal("error", state.error.signal())
                .event(clone!(state => move |evt: events::CustomInput| {
                    submit_code(Rc::clone(&state), evt.value());
                }))
            }))
            .child(html!("kids-student-code-jigzi", {
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
                let mut player_options: JigPlayerOptions = settings.into();
                player_options.is_student = true;
                PlayerPopup::render(
                    Rc::new(PlayerPopup::new(jig_id, player_options, PreviewPopupCallbacks::new(close))),
                    None
                )
            })
        })))
    })
}
