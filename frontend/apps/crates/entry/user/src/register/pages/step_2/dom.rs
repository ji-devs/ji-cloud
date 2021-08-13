use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use crate::{
    strings::register::step_2::*,
    register::{
        state::{Step, Step1Data},
        components::footer::Footer
    }
};

pub struct Step2Page {
}

impl Step2Page {
    pub fn render(step: Mutable<Step>, step_1: Step1Data) -> Dom {
        let state = Rc::new(State::new(step, step_1));

        html!("page-register-step2", {
            .children(&mut [
                html!("input-wrapper", {
                    .property("slot", "location")
                    .property("label", STR_LOCATION_LABEL)
                    .property_signal("error", state.location_error.signal())
                    .child(html!("input-location", {
                        .event(clone!(state => move |evt:events::GoogleLocation| {
                            *state.location_json.borrow_mut() = evt.raw_json();
                        }))
                    }))
                }),
                html!("input-wrapper", {
                    .property("slot", "language")
                    .property("label", STR_LANGUAGE_LABEL)
                    .child(html!("input", {
                        .event(clone!(state => move |evt:events::Input| {
                            *state.language.borrow_mut() = evt.value().unwrap_or_default();
                        }))
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

