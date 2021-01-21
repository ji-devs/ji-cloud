use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use crate::register::{
    state::{Step, Step1Data},
    components::footer::Footer
};

const STR_FIRSTNAME_LABEL:&'static str = "First name";
const STR_FIRSTNAME_PLACEHOLDER:&'static str = "Type your first name";
const STR_LASTNAME_LABEL:&'static str = "Last name";
const STR_LASTNAME_PLACEHOLDER:&'static str = "Type your last name";
const STR_USERNAME_LABEL:&'static str = "Create a User Name*";
const STR_USERNAME_PLACEHOLDER:&'static str = "This will be your public name on JI";
const STR_18:&'static str = "I am over 18";
const STR_CONTINUE:&'static str = "Continue";

pub struct Step1Page {
}

impl Step1Page {
    pub fn render(step: Mutable<Step>, init_data: Step1Data) -> Dom {
        let state = Rc::new(State::new(step, init_data));

        html!("page-register-step1", {
            .children(vec![
                html!("input-text", {
                    .property("slot", "topleft")
                    .property("label", STR_FIRSTNAME_LABEL)
                    .property("placeholder", STR_FIRSTNAME_PLACEHOLDER)
                    .property_signal("error", state.firstname_error())
                    .event(clone!(state => move |evt:events::CustomInput| {
                        state.clear_firstname_status();
                        *state.firstname.borrow_mut() = evt.value();
                    }))
                }),
                html!("input-text", {
                    .property("slot", "topright")
                    .property("label", STR_LASTNAME_LABEL)
                    .property("placeholder", STR_LASTNAME_PLACEHOLDER)
                    .property_signal("error", state.lastname_error())
                    .event(clone!(state => move |evt:events::CustomInput| {
                        state.clear_lastname_status();
                        *state.lastname.borrow_mut() = evt.value();
                    }))
                }),
                html!("input-text", {
                    .property("slot", "username")
                    .property("label", STR_USERNAME_LABEL)
                    .property("placeholder", STR_USERNAME_PLACEHOLDER)
                    .property_signal("error", state.username_error())
                    .event(clone!(state => move |evt:events::CustomInput| {
                        state.clear_username_status();
                        *state.username.borrow_mut() = evt.value();
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
                html!("button-rect", {
                    .property("slot", "submit")
                    .property("iconAfter", "arrow")
                    .property("color", "red")
                    .property("size", "medium")
                    .text(STR_CONTINUE)
                    .event(clone!(state => move |evt:events::Click| {
                        actions::next_step(state.clone());
                    }))
                }),
                Footer::render()
            ])
        })
    }
}
