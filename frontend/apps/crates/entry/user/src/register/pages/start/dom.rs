use super::state::RegisterStart;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use web_sys::HtmlInputElement;

use crate::{register::components::footer::Footer, strings};
use utils::events;

const STR_GOOGLE_LABEL: &str = "Sign up with Google";
const STR_REGISTER_FIRST: &str = "Please register before logging in";

impl RegisterStart {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("empty-fragment", {
            .child(html!("window-loader-block", {
                .property_signal("visible", state.loader.is_loading())
            }))
            .child(html!("page-register-start", {
                .apply_if(state.login_before_register, |dom| {
                    dom.child(html!("p", {
                        .property("slot", "alert")
                        .text(STR_REGISTER_FIRST)
                    }))
                })
                .property_signal("passwordStrength", state.password.strength_signal())
                .children(vec![
                    html!("input-wrapper", {
                        .property("slot", "email")
                        .property("label", strings::STR_EMAIL_LABEL)
                        .property_signal("error", state.show_email_error_signal().map(|err| {
                            err.is_some()
                        }))
                        .property_signal("hint", state.show_email_error_signal())
                        .child(html!("input" => HtmlInputElement, {
                            .with_node!(elem => {
                                .property("type", "email")
                                .attribute("autocomplete", "email")
                                .event(clone!(state => move |_:events::Input| {
                                    state.email.update_value(elem.value());
                                }))
                            })
                        }))
                    }),
                    html!("input-password", {
                        .property("slot", "password")
                        .property("label", strings::STR_PASSWORD_CREATE_LABEL)
                        .property("placeholder", strings::STR_PASSWORD_PLACEHOLDER)
                        .property("autocomplete", "new-password")
                        .property_signal("error", state.show_password_error_signal().map(|err| {
                            err.is_some()
                        }))
                        .property_signal("hint", state.show_password_error_signal())
                        .event(clone!(state => move |evt:events::CustomInput| {
                            state.password.update_value(evt.value());
                        }))
                    }),
                    html!("button-google", {
                        .property("slot", "google")
                        .property("label", STR_GOOGLE_LABEL)
                        .event(clone!(state => move |_evt:events::Click| {
                            state.register_google()
                        }))
                    }),
                    html!("button-rect-icon", {
                        .property("slot", "submit")
                        .property("color", "red")
                        .property("size", "medium")
                        .property("iconAfter", "arrow")
                        .text(strings::STR_CONTINUE)
                        .event(clone!(state => move |_evt:events::Click| {
                            state.register_email()
                        }))
                    }),
                    Footer::render(),
                ])
            }))
        })
    }
}
