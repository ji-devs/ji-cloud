use crate::login::send_reset_link::{SendResetLink, SendResetLinkCallbacks};

use super::{actions, state::LoginPage};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use web_sys::HtmlInputElement;

use utils::events;

const STR_GOOGLE_LABEL: &str = "Log in with Google";
const STR_BASIC_TRIED_OAUTH: &str =
    "Looks like you didn't sign up with Google, try logging in with a password";

impl LoginPage {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        html!("empty-fragment", {
            .child_signal(state.reset_password_popup.signal().map(clone!(state => move |reset_password_popup| {
                match reset_password_popup {
                    false => None,
                    true => {
                        let callbacks = SendResetLinkCallbacks::new(
                            clone!(state => move|| {
                                state.reset_password_popup.set(false);
                            })
                        );
                        Some(SendResetLink::new(callbacks).render())
                    },
                }
            })))
            .child(html!("window-loader-block", {
                .prop_signal("visible", state.loader.is_loading())
            }))
            .child(html!("page-login-landing", {
                .apply_if(state.basic_tried_oauth, |dom| {
                    dom.child(html!("p", {
                        .prop("slot", "alert")
                        .text(STR_BASIC_TRIED_OAUTH)
                    }))
                })
                .children(vec![
                    html!("input-wrapper", {
                        .prop("slot", "email")
                        .prop("label", crate::strings::STR_EMAIL_LABEL)
                        .prop_signal("hint", state.show_email_error_signal())
                        .prop_signal("error", state.show_email_error_signal().map(|err| {
                            err.is_some()
                        }))
                        .child(html!("input" => HtmlInputElement, {
                            .with_node!(elem => {
                                .prop("type", "email")
                                .attr("autocomplete", "email")
                                .event(clone!(state => move |_:events::Input| {
                                    state.email.update_value(elem.value());
                                }))
                            })
                        }))
                    }),
                    html!("input-password", {
                        .prop("slot", "password")
                        .prop("label", crate::strings::STR_PASSWORD_LABEL)
                        .prop("placeholder", crate::strings::STR_PASSWORD_PLACEHOLDER)
                        .prop_signal("hint", state.password_error.signal_cloned())
                        .prop_signal("error", state.password_error.signal_ref(|err| {
                            err.is_some()
                        }))
                        .event(clone!(state => move |evt:events::CustomInput| {
                            *state.password.borrow_mut() = evt.value();
                        }))
                    }),
                    html!("button-google", {
                        .prop("slot", "google")
                        .prop("label", STR_GOOGLE_LABEL)
                        .event(clone!(state => move |_evt:events::Click| {
                            actions::signin_google(state.clone())
                        }))
                    }),
                    html!("button-rect", {
                        .prop("slot", "password-forgot")
                        .prop("kind", "text")
                        .prop("color", "blue")
                        .text(crate::strings::STR_PASSWORD_FORGOTTEN)
                        .event(clone!(state => move |_evt:events::Click| {
                            state.reset_password_popup.set(true);
                        }))
                    }),
                    html!("button-rect-icon", {
                        .prop("slot", "submit")
                        .prop("color", "red")
                        .prop("size", "medium")
                        .prop("iconAfter", "arrow")
                        .text(crate::strings::STR_CONTINUE)
                        .event(clone!(state => move |_evt:events::Click| {
                            actions::signin_email(state.clone())
                        }))
                    }),
                    html!("footer-login-register", {
                        .prop("slot", "footer")
                        .event(clone!(state => move |_evt:events::Click| {
                            actions::go_register(state.clone())
                        }))
                    }),
                ])
            }))
        })
    }
}
