use super::{actions, state::*};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use web_sys::HtmlInputElement;

use crate::{
    register::{components::footer::Footer, state::Step},
    strings,
};
use utils::events;

const STR_GOOGLE_LABEL: &str = "Sign up with Google";

pub struct StartPage {}

impl StartPage {
    pub fn render(step: Mutable<Step>, _is_no_auth: bool) -> Dom {
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
                        .property("label", strings::STR_PASSWORD_CREATE_LABEL)
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
                        .property("label", STR_GOOGLE_LABEL)
                        .event(clone!(state => move |_evt:events::Click| {
                            actions::register_google(state.clone())
                        }))
                    }),
                    html!("button-rect-icon", {
                        .property("slot", "submit")
                        .property("color", "red")
                        .property("size", "medium")
                        .property("iconAfter", "arrow")
                        .text(strings::STR_CONTINUE)
                        .event(clone!(state => move |_evt:events::Click| {
                            actions::register_email(state.clone())
                        }))
                    }),
                    Footer::render(),
                ])
            }))
        })
    }
}
