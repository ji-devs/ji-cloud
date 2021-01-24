use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use crate::firebase::*;
use crate::register::{
    state::{Step, Step1Data},
    components::footer::Footer
};

const STR_SUBMIT:&'static str = "Submit";
const STR_LOCATION_LABEL:&'static str = "Location*";
const STR_TERMS_LABEL:&'static str = "I have read the terms and conditions (legal text…)";
const STR_LANGUAGE_LABEL:&'static str = "Preferred language of communication*";
const STR_MARKETING_LABEL:&'static str = "I would like to receive educational resources (GDPR legal text….)";

pub struct Step2Page {
}

impl Step2Page {
    pub fn render(step: Mutable<Step>, step_1: Step1Data) -> Dom {
        let state = Rc::new(State::new(step, step_1));

        html!("page-register-step2", {
            .children(&mut [
                html!("input-location", {
                    .property("slot", "location")
                    .event(clone!(state => move |evt:events::GoogleLocation| {
                        *state.location_json.borrow_mut() = evt.raw_json();
                    }))
                }),
                html!("input-text", {
                    .property("slot", "language")
                    .property("label", STR_LANGUAGE_LABEL)
                    .property("mode", "text")
                    .event(clone!(state => move |evt:events::CustomInput| {
                        *state.language.borrow_mut() = evt.value();
                    }))
                }),
                html!("input-checkbox", {
                    .property("slot", "checkbox")
                    .property_signal("error", state.terms_error())
                    .property("label", STR_TERMS_LABEL)
                    .event(clone!(state => move |evt:events::CustomToggle| {
                        state.clear_terms_status();
                        *state.terms.borrow_mut() = evt.value();
                    }))
                }),
                html!("input-checkbox", {
                    .property("slot", "checkbox")
                    .property("label", STR_MARKETING_LABEL)
                    .event(clone!(state => move |evt:events::CustomToggle| {
                        *state.marketing.borrow_mut() = evt.value();
                    }))
                }),
                html!("button-rect", {
                    .property("slot", "submit")
                    .property("color", "red")
                    .property("size", "medium")
                    .text(STR_SUBMIT)
                    .event(clone!(state => move |evt:events::Click| {
                        actions::submit(state.clone());
                    }))
                }),
                Footer::render()
            ])
        })
            
    }
}

