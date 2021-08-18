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

use components::input::simple_select::SimpleSelect;

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
                SimpleSelect::render_mixin(
                    SimpleSelect::new(
                        Some(STR_LANGUAGE_LABEL),
                        Some(STR_LANGUAGE_PLACEHOLDER),
                        None,
                        STR_LANGUAGE_OPTIONS.to_vec(), 
                        clone!(state => move |value| {
                            *state.language.borrow_mut() = value.map(|x| x.to_string()); 
                            state.evaluate_language_error();
                        })
                    ),
                    Some("language"),
                    clone!(state => move |dom| {
                        dom.property_signal("error", state.language_error.signal())
                    })
                ),
                SimpleSelect::render_mixin(
                    SimpleSelect::new(
                        Some(STR_PERSONA_LABEL),
                        Some(STR_PERSONA_PLACEHOLDER),
                        None,
                        STR_PERSONA_OPTIONS.to_vec(), 
                        clone!(state => move |value| {
                            *state.persona.borrow_mut() = value.map(|x| x.to_string()); 
                            state.evaluate_persona_error();
                        })
                    ),
                    Some("persona"),
                    clone!(state => move |dom| {
                        dom.property_signal("error", state.persona_error.signal())
                    })
                ),
                html!("input-wrapper", {
                    .property("slot", "organization")
                    .property("label", STR_ORGANIZATION_LABEL)
                    .child(html!("input", {
                        .event(clone!(state => move |evt:events::Input| {
                            *state.organization.borrow_mut() = evt.value().and_then(|x| {
                                if x.is_empty() { None } else { Some(x) }
                            })
                        }))
                    }))
                }),
                html!("input-checkbox", {
                    .property("slot", "checkbox")
                    .property_signal("error", state.terms_error_str())
                    .property("label", STR_TERMS_LABEL)
                    .event(clone!(state => move |evt:events::CustomToggle| {
                        *state.terms.borrow_mut() = evt.value();
                        state.evaluate_terms_error();
                    }))
                }),
                html!("input-checkbox", {
                    .property("slot", "checkbox")
                    .property("label", STR_MARKETING_LABEL)
                    .event(clone!(state => move |evt:events::CustomToggle| {
                        *state.marketing.borrow_mut() = evt.value();
                    }))
                }),
                html!("button-rect-icon", {
                    .property("slot", "submit")
                    .property("color", "red")
                    .property("size", "medium")
                    .property("iconAfter", "arrow")
                    .text(STR_ONE_MORE_STEP)
                    .event(clone!(state => move |evt:events::Click| {
                        actions::submit(state.clone());
                    }))
                }),
                Footer::render()
            ])
        })
            
    }
}

