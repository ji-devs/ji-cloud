use super::{actions, state::*};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use web_sys::HtmlInputElement;

use crate::{register::state::Step, strings::register::step_1::*};
use shared::domain::session::OAuthUserProfile;
use utils::events;

pub struct Step1Page {}

impl Step1Page {
    pub fn render(step: Mutable<Step>, oauth_profile: Option<OAuthUserProfile>) -> Dom {
        let state = Rc::new(State::new(step, oauth_profile));

        html!("page-register-step1", {
            .children(vec![
                html!("input-wrapper", {
                    .property("slot", "topleft")
                    .property("label", STR_FIRSTNAME_LABEL)
                    .property_signal("error", state.firstname_error().map(|err| {
                        !err.is_empty()
                    }))
                    .property_signal("hint", state.firstname_error())
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .property("value", &*state.firstname.borrow())
                            .event(clone!(state, elem => move |_:events::Input| {
                                state.clear_firstname_status();
                                *state.firstname.borrow_mut() = elem.value();
                            }))
                        })
                    }))
                }),
                html!("input-wrapper", {
                    .property("slot", "topright")
                    .property("label", STR_LASTNAME_LABEL)
                    .property_signal("error", state.lastname_error().map(|err| {
                        !err.is_empty()
                    }))
                    .property_signal("hint", state.lastname_error())
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .property("value", &*state.lastname.borrow())
                            .event(clone!(state => move |_:events::Input| {
                                state.clear_lastname_status();
                                *state.lastname.borrow_mut() = elem.value();
                            }))
                        })
                    }))
                }),
                html!("input-wrapper", {
                    .property("slot", "username")
                    .property("label", STR_USERNAME_LABEL)
                    .property_signal("error", state.username_error().map(|err| {
                        !err.is_empty()
                    }))
                    .property_signal("hint", state.username_error())
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .property("placeholder", STR_USERNAME_PLACEHOLDER)
                            .property("value", &*state.username.borrow())
                            .event(clone!(state => move |_:events::Input| {
                                state.clear_username_status();
                                *state.username.borrow_mut() = elem.value();
                            }))
                        })
                    }))
                }),
                html!("input-checkbox", {
                    .property("slot", "checkbox")
                    .property("label", STR_18)
                    .property_signal("error", state.over_18_error())
                    .event(clone!(state => move |evt:events::CustomToggle| {
                        state.clear_over_18_status();
                        *state.over_18.borrow_mut() = evt.value();
                    }))
                }),
                html!("button-rect-icon", {
                    .property("slot", "submit")
                    .property("color", "red")
                    .property("size", "medium")
                    .property("iconAfter", "arrow")
                    .text(STR_NEXT)
                    .event(clone!(state => move |_evt:events::Click| {
                        actions::submit(state.clone());
                    }))
                }),
            ])
        })
    }
}
