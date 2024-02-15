use std::{cell::RefCell, rc::Rc};

use components::player_popup::{PlayerPopup, PreviewPopupCallbacks};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
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
                                    None if play_jig.settings.display_score => {
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
        let name = play_jig.name.clone();
        let input: Rc<RefCell<Option<HtmlInputElement>>> = Rc::new(RefCell::new(None));
        html!("kids-student-code-name", {
            .children(&mut [
                html!("input" => HtmlInputElement, {
                    .prop("slot", "input")
                    .prop("placeholder", "Type your name")
                    .after_inserted(clone!(input => move |el| {
                        *input.borrow_mut() = Some(el);
                    }))
                }),
                html!("button", {
                    .prop("slot", "clear")
                    .child(html!("span", {
                        .text("×")
                        .style("font-size", "30px")
                    }))
                    .text("Clear")
                    .event(clone!(input => move |_: events::Click| {
                        input.borrow().as_ref().map(|input| input.set_value(""));
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
                    .event(clone!(input => move |_: events::Click| {
                        let value = input.borrow().as_ref().map(|input| input.value()).unwrap_or_default();
                        name.set(Some(value));
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
        let mut player_options: JigPlayerOptions = play_jig.settings.into();
        player_options.is_student = true;
        player_options.play_token = Some(play_jig.token);
        player_options.players_name = play_jig.name.get_cloned();

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
