use std::rc::Rc;

use components::player_popup::{PlayerPopup, PreviewPopupCallbacks};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use utils::{asset::JigPlayerOptions, events};

use super::state::StudentCode;

const STR_TRY_AGAIN: &str = "Try again";
const STR_HELP: &str = "Ask for help";

impl StudentCode {
    pub fn render(self: Rc<Self>, code: Option<String>) -> Dom {
        let state = self;
        if let Some(code) = code {
            state.submit_code(code);
        };

        html!("empty-fragment", {
            .child(html!("kids-student-code", {
                .child(html!("kids-student-code-input", {
                    .prop("slot", "input")
                    .prop_signal("error", state.error.signal())
                    .event(clone!(state => move |evt: events::CustomInput| {
                        state.submit_code(evt.value());
                    }))
                }))
                .child(html!("kids-student-code-jigzi", {
                    .prop("slot", "jigzi")
                    .prop_signal("mode", state.error.signal().map(|error| {
                        match error {
                            true => "try-again",
                            false => "default",
                        }
                    }))
                    .child(html!("button", {
                        .prop("slot", "try-again")
                        .text(STR_TRY_AGAIN)
                    }))
                    .child(html!("button", {
                        .prop("slot", "help")
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
                    PlayerPopup::new(
                        jig_id.into(),
                        None,
                        None,
                        player_options.into(),
                        PreviewPopupCallbacks::new(close)
                    ).render(None)
                })
            })))
        })
    }
}
