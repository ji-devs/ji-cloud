use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*};

const STR_PASSWORD_LABEL:&'static str = "Password";
const STR_FORGOTTEN:&'static str ="Forgot your Password?";
const STR_EMAIL_LABEL:&'static str = "Email";
const STR_SUBMIT:&'static str = "Submit";


pub struct LoginPage {
}
impl LoginPage {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        html!("empty-fragment", {
            .child(html!("window-loader-block", {
                .property_signal("visible", state.loader.is_loading())
            }))
            .child(html!("page-login-landing", {
                .future(state.status.signal_cloned().for_each(|status| {
                    actions::status_redirect(status);
                    async {}
                }))
                .children(vec![
                    html!("input-wrapper", {
                        .property("slot", "email")
                        .property("label", STR_EMAIL_LABEL)
                        .property_signal("hint", state.email_error())
                        .property_signal("error", state.email_error().map(|err| {
                            !err.is_empty()
                        }))
                        .child(html!("input", {
                            .event(clone!(state => move |evt:events::Input| {
                                state.clear_email_status();
                                *state.email.borrow_mut() = evt.value().unwrap_or_default();
                            }))
                        }))
                    }),
                    html!("input-password", {
                        .property("slot", "password")
                        .property("label", STR_PASSWORD_LABEL)
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
                        .event(clone!(state => move |evt:events::Click| {
                            actions::signin_google(state.clone())
                        }))
                    }),
                    html!("button-rect", {
                        .property("slot", "password-forgot")
                        .property("kind", "text")
                        .property("color", "blue")
                        .text(STR_FORGOTTEN)
                        .event(clone!(state => move |evt:events::Click| {
                            actions::forgot_password(state.clone())
                        }))
                    }),
                    html!("button-rect", {
                        .property("slot", "submit")
                        .property("color", "red")
                        .property("size", "medium")
                        .text(STR_SUBMIT)
                        .event(clone!(state => move |evt:events::Click| {
                            actions::signin_email(state.clone())
                        }))
                    }),
                    html!("footer-login-register", {
                        .property("slot", "footer")
                        .event(clone!(state => move |evt:events::Click| {
                            actions::go_register(state.clone())
                        }))
                    }),
                ])
            }))
        })

    }
}

