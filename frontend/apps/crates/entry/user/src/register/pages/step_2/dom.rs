use super::{actions, state::*};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use web_sys::HtmlInputElement;

use crate::{
    register::{
        components::footer::Footer,
        state::{Step, Step1Data},
    },
    strings::register::step_2::*,
};
use utils::{events, languages};

use super::Language;

use components::input::simple_select::{SimpleSelect, SimpleSelectItem};

pub struct Step2Page {}

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
                        .property("placeholder", STR_LOCATION_PLACEHOLDER)
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
                        languages::EMAIL_LANGUAGES.iter().map(|l| Language::from(l.clone())).collect(),
                        clone!(state => move |value| {
                            *state.language.borrow_mut() = value.map(|lang| lang.value().to_string());
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
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .event(clone!(state => move |_:events::Input| {
                                let value = elem.value();
                                *state.organization.borrow_mut() = if value.is_empty() {
                                    None
                                } else {
                                    Some(value)
                                }
                            }))
                        })
                    }))
                }),
                html!("div", {
                    .style("display", "flex")
                    .property("slot", "checkbox")
                    .child(
                        html!("input-checkbox", {
                            .property_signal("error", state.terms_error_str())
                            .property("label", STR_TERMS_LABEL_ONE)
                            .event(clone!(state => move |evt:events::CustomToggle| {
                                *state.terms.borrow_mut() = evt.value();
                                state.evaluate_terms_error();
                            }))
                        })
                    )
                    .child(
                        html!("div", {
                            //whatever.. good enough for now
                            .style("gap", ".5em")
                            .style("margin-left", ".5em")
                            .child(html!("a", {
                                .attribute("href", "https://www.jewishinteractive.org/terms-and-conditions/")
                                .text(STR_TERMS_TERMS)
                                .attribute("target", "_blank")
                                .style("color", "var(--main-blue)")
                            }))
                            .child(html!("span", {
                                .text(STR_TERMS_LABEL_TWO)
                            }))
                            .child(html!("a", {
                                .attribute("target", "_blank")
                                .attribute("href", "https://www.jewishinteractive.org/privacy-policy/")
                                .text(STR_TERMS_PRIVACY)
                                .style("color", "var(--main-blue)")
                            }))
                            .text(". *")
                        })
                    )
                }),
                html!("input-checkbox", {
                    .property("slot", "checkbox")
                    .property("label", STR_MARKETING_LABEL)
                    .event(clone!(state => move |evt:events::CustomToggle| {
                        *state.marketing.borrow_mut() = evt.value();
                    }))
                }),
                html!("p", {
                    .property("slot", "committed-to-privacy")
                    .text(STR_PROTECTING_PRIVACY)
                }),
                html!("button-rect-icon", {
                    .property("slot", "submit")
                    .property("color", "red")
                    .property("size", "medium")
                    .property("iconAfter", "arrow")
                    .text(STR_ONE_MORE_STEP)
                    .event(clone!(state => move |_evt:events::Click| {
                        actions::submit(state.clone());
                    }))
                }),
                Footer::render()
            ])
        })
    }
}
