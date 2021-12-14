use super::{actions, state::*};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use web_sys::HtmlInputElement;

use utils::events;

const STR_GOOGLE_LABEL: &str = "Log in with Google";

pub struct LoginPage {}
impl LoginPage {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        html!("empty-fragment", {
            .child(html!("window-loader-block", {
                .property_signal("visible", state.loader.is_loading())
            }))
            .child(html!("page-login-landing", {
                .children(vec![
                    html!("input-wrapper", {
                        .property("slot", "email")
                        .property("label", crate::strings::STR_EMAIL_LABEL)
                        .property_signal("hint", state.email_error())
                        .property_signal("error", state.email_error().map(|err| {
                            !err.is_empty()
                        }))
                        .child(html!("input" => HtmlInputElement, {
                            .with_node!(elem => {
                                .property("type", "email")
                                .attribute("autocomplete", "email")
                                .event(clone!(state => move |_:events::Input| {
                                    state.clear_email_status();
                                    *state.email.borrow_mut() = elem.value();
                                }))
                            })
                        }))
                    }),
                    html!("input-password", {
                        .property("slot", "password")
                        .property("label", crate::strings::STR_PASSWORD_LABEL)
                        .property("placeholder", crate::strings::STR_PASSWORD_PLACEHOLDER)
                        .property_signal("hint", state.password_error())
                        .property_signal("error", state.password_error().map(|err| {
                            !err.is_empty()
                        }))
                        .event(clone!(state => move |evt:events::CustomInput| {
                            state.clear_password_status();
                            *state.password.borrow_mut() = evt.value();
                        }))
                    }),
                    html!("button-google", {
                        .property("slot", "google")
                        .property("label", STR_GOOGLE_LABEL)
                        .event(clone!(state => move |_evt:events::Click| {
                            actions::signin_google(state.clone())
                        }))
                    }),
                    html!("button-rect", {
                        .property("slot", "password-forgot")
                        .property("kind", "text")
                        .property("color", "blue")
                        .text(crate::strings::STR_PASSWORD_FORGOTTEN)
                        .event(clone!(state => move |_evt:events::Click| {
                            actions::forgot_password(state.clone())
                        }))
                    }),
                    html!("button-rect-icon", {
                        .property("slot", "submit")
                        .property("color", "red")
                        .property("size", "medium")
                        .property("iconAfter", "arrow")
                        .text(crate::strings::STR_CONTINUE)
                        .event(clone!(state => move |_evt:events::Click| {
                            actions::signin_email(state.clone())
                        }))
                    }),
                    html!("footer-login-register", {
                        .property("slot", "footer")
                        .event(clone!(state => move |_evt:events::Click| {
                            actions::go_register(state.clone())
                        }))
                    }),
                ])
            }))
        })
    }
}
