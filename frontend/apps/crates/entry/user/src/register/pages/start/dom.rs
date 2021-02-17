use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use crate::register::{
    state::Step,
    components::footer::Footer
};

const STR_SUBMIT:&'static str = "Submit";
const STR_EMAIL_LABEL:&'static str = "Email";
const STR_EMAIL_PLACEHOLDER:&'static str = "Type or paste your email";
const STR_PASSWORD_LABEL:&'static str = "Create Password";
const STR_PASSWORD_PLACEHOLDER:&'static str ="********";

pub struct StartPage {
}

impl StartPage {
    pub fn render(step: Mutable<Step>) -> Dom {
        let state = Rc::new(State::new(step));

        html!("page-register-start", {
            .property_signal("passwordStrength", state.get_password_strength())
            .global_event(clone!(state => move |evt:dominator_helpers::events::Message| {
                actions::handle_window_message(state.clone(), evt);
            }))
            .children(vec![
                html!("input-text", {
                    .property("slot", "email")
                    .property("mode", "text")
                    .property("label", STR_EMAIL_LABEL)
                    .property("placeholder", STR_EMAIL_PLACEHOLDER)
                    .property_signal("error", state.email_error())
                    .event(clone!(state => move |evt:events::CustomInput| {
                        state.clear_email_status();
                        *state.email.borrow_mut() = evt.value();
                    }))
                }),
                html!("input-text", {
                    .property("slot", "password")
                    .property("mode", "passwordHidden")
                    .property("label", STR_PASSWORD_LABEL)
                    .property("placeholder", STR_PASSWORD_PLACEHOLDER)
                    .property_signal("error", state.password_error())
                    .event(clone!(state => move |evt:events::CustomInput| {
                        state.clear_password_status();
                        *state.password.borrow_mut() = evt.value();
                        actions::update_password_strength(&state);
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
                    .text(STR_SUBMIT)
                    .event(clone!(state => move |evt:events::Click| {
                        actions::register_email(state.clone())
                    }))
                }),
                Footer::render()
            ])
        })
    }
}

