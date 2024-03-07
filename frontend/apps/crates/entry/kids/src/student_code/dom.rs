use std::rc::Rc;

use components::player_popup::{PlayerPopup, PreviewPopupCallbacks};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use shared::domain::asset::DraftOrLive;
use utils::{asset::JigPlayerOptions, events};
use web_sys::HtmlInputElement;

use super::state::{PlayJig, StudentCode};

const STR_TRY_AGAIN: &str = "Try again";
const STR_HELP: &str = "Ask for help";

impl StudentCode {
    pub fn render(self: Rc<Self>, code: Option<String>) -> Dom {
        let state = self;
        if let Some(code) = code {
            state.submit_code(code);
        };

        html!("div", {
            .child_signal(state.play_jig.signal_cloned().map(clone!(state => move|play_jig| {
                Some(match play_jig {
                    None => state.render_code_input(),
                    Some(play_jig) => {
                        html!("div", {
                            .child_signal(play_jig.name.signal_cloned().map(clone!(state => move |name| {
                                Some(match name {
                                    None if play_jig.settings.scoring => {
                                        state.render_name_input(play_jig.clone())
                                    },
                                    _ => {
                                        state.render_jig(play_jig.clone())
                                    },
                                })
                            })))
                        })
                    },
                })
            })))
        })
    }

    fn render_code_input(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("kids-student-code", {
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
        })
    }

    fn render_name_input(self: &Rc<Self>, play_jig: PlayJig) -> Dom {
        // name after finish typing
        let final_name = play_jig.name.clone();
        // name signal that changes as the user types.
        let local_name = Mutable::new(String::new());

        html!("kids-student-code-name", {
            .children(&mut [
                html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .prop("slot", "input")
                        .prop("placeholder", "Type your name")
                        .prop_signal("value", local_name.signal_cloned())
                        .event(clone!(local_name => move |_: events::Input| {
                            local_name.set(elem.value());
                        }))
                    })
                }),
                html!("button", {
                    .prop("slot", "clear")
                    .child(html!("span", {
                        .text("×")
                        .style("font-size", "30px")
                    }))
                    .text("Clear")
                    .event(clone!(local_name => move |_: events::Click| {
                        local_name.set(String::new())
                    }))
                }),
                html!("button-rect", {
                    .prop("slot", "play")
                    .prop("color", "red")
                    .prop("size", "large")
                    .text("Play")
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-play")
                        .style("color", "var(--main-yellow)")
                        .style("font-size", "20px")
                    }))
                    .prop_signal("disabled", local_name.signal_ref(|name| name.is_empty()))
                    .event(clone!(local_name => move |_: events::Click| {
                        let local_name = local_name.get_cloned();
                        if !local_name.is_empty() {
                            final_name.set(Some(local_name));
                        }
                    }))
                }),
            ])
        })
    }

    fn render_jig(self: &Rc<Self>, play_jig: PlayJig) -> Dom {
        let state = self;
        let close = clone!(state => move || {
            state.play_jig.set(None);
        });

        let player_options = JigPlayerOptions {
            draft_or_live: DraftOrLive::Live,
            play_token: Some(play_jig.token),
            players_name: play_jig.name.get_cloned(),
            is_student: true,
            quota: false,
            direction: Some(play_jig.settings.direction),
            scoring: Some(play_jig.settings.scoring),
            drag_assist: Some(play_jig.settings.drag_assist),
        };

        PlayerPopup::new(
            play_jig.id.into(),
            None,
            None,
            player_options.into(),
            PreviewPopupCallbacks::new(close),
        )
        .render(None)
    }
}
