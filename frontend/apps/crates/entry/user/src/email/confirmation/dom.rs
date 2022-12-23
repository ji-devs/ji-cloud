use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

const STR_DIDNT_RECEIVE: &str = "Still nothing? ";
const STR_SEND_AGAIN: &str = "Send again";

impl SendEmailConfirmationPage {
    pub fn render(state: Rc<SendEmailConfirmationPage>) -> Dom {
        html!("page-email-send", {
            .prop("email", &state.email)
            .child(html!("window-loader-block", {
                .prop_signal("visible", state.loader.is_loading())
            }))
            //this doesn't actually make sense here... the *only* thing we have is their email
            //it's equivilent to just creating a new account which is simpler
            /*
            .child(html!("button-rect", {
                .prop("slot", "change")
                .prop("color", "blue")
                .text(crate::strings::STR_CHANGE_EMAIL)
            }))
            */
            .child_signal(state.mode.signal().map(clone!(state => move |mode| {
                match mode {
                    Mode::Send => {
                        Some(html!("p", {
                            .prop("slot", "send")
                            .text(STR_DIDNT_RECEIVE)
                            .child(html!("button-rect", {
                                .prop("color", "blue")
                                .prop("kind", "text")
                                .text(STR_SEND_AGAIN)
                                .event(clone!(state => move |_:events::Click| {
                                    state.resend();
                                }))
                            }))
                        }))
                    },
                    Mode::Sent => {
                        Some(html!("button-email-send", {
                            .prop("slot", "send")
                            .prop_signal("mode", state.mode_str())
                            .event(clone!(state => move |_:events::Click| {
                                state.resend();
                            }))
                        }))
                    },
                }
            })))
        })
    }
}
