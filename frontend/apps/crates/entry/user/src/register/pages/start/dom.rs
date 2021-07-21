use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use crate::{strings, register::{
    state::Step,
    components::footer::Footer
}};


pub struct StartPage {
}

impl StartPage {
    pub fn render(step: Mutable<Step>) -> Dom {
        let state = Rc::new(State::new(step));

        html!("empty-fragment", {
            .child(html!("window-loader-block", {
                .property_signal("visible", state.loader.is_loading())
            }))
            .child(html!("page-register-start", {
                .property_signal("passwordStrength", state.password.get_strength())
                .children(vec![
                    html!("input-wrapper", {
                        .property("slot", "email")
                        .property("label", strings::STR_EMAIL_LABEL)
                        .property_signal("error", state.email_error().map(|err| {
                            !err.is_empty()
                        }))
                        .property_signal("hint", state.email_error())
                        .child(html!("input", {
                            .property("type", "email")
                            .attribute("autocomplete", "email")
                            .property("placeholder", strings::STR_EMAIL_PLACEHOLDER)
                            .event(clone!(state => move |evt:events::Input| {
                                state.clear_email_status();
                                *state.email.borrow_mut() = evt.value().unwrap_or_default();
                            }))
                        }))
                    }),
                    html!("input-password", {
                        .property("slot", "password")
                        .property("label", strings::STR_PASSWORD_LABEL)
                        .property("placeholder", strings::STR_PASSWORD_PLACEHOLDER)
                        .property("autocomplete", "new-password")
                        .property_signal("error", state.password.error().map(|err| {
                            !err.is_empty()
                        }))
                        .property_signal("hint", state.password.error())
                        .event(clone!(state => move |evt:events::CustomInput| {
                            state.password.clear_status();
                            *state.password.value.borrow_mut() = evt.value();
                            state.password.update_strength();
                        }))
                    }),
                    html!("button-google", {
                        .property("slot", "google")
                        .event(clone!(state => move |evt:events::Click| {
                            actions::register_google(state.clone())
                        }))
                    }),
                    html!("button-rect", {
                        .property("slot", "submit")
                        .property("color", "red")
                        .property("size", "medium")
                        .text(strings::STR_SUBMIT)
                        .event(clone!(state => move |evt:events::Click| {
                            actions::register_email(state.clone())
                        }))
                    }),
                    Footer::render(),
                ])
            }))
        })
    }
}

