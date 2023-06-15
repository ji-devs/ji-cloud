use super::{actions, state::*};
use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::SignalVecExt,
};
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
                    .prop("slot", "location")
                    .prop("label", STR_LOCATION_LABEL)
                    .prop_signal("error", state.location_error.signal())
                    .child(html!("input-location", {
                        .prop("placeholder", STR_LOCATION_PLACEHOLDER)
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
                        dom.prop_signal("error", state.language_error.signal())
                    })
                ),
                html!("input-select", {
                    .prop("slot", "persona")
                    .prop("label", STR_PERSONA_LABEL)
                    .prop("placeholder", STR_PERSONA_PLACEHOLDER)
                    .prop("multiple", true)
                    .prop_signal("value", state.persona.signal_vec_cloned().to_signal_cloned().map(|persona| {
                        persona.join(", ")
                    }))
                    .children(STR_PERSONA_OPTIONS.iter().map(|persona| {
                        html!("input-select-option", {
                            .text(persona)
                            .prop_signal(
                                "selected",
                                state.persona.signal_vec_cloned().to_signal_cloned().map(move |p| {
                                    p.iter().any(|p| p == persona)
                                })
                            )
                            .event(clone!(state => move |evt: events::CustomSelectedChange| {
                                let pos = state.persona.lock_ref().iter().position(|p| p == persona);

                                if evt.selected() {
                                    if pos.is_none() {
                                        // Only add the selection if it doesn't exist yet and the
                                        // event is selected.
                                        state.persona.lock_mut().push_cloned(persona.to_string());
                                    }
                                } else if let Some(pos) = pos {
                                    // Only remove the selection if it does exist and the event
                                    // is not selected.
                                    state.persona.lock_mut().remove(pos);
                                }

                                state.evaluate_persona_error();
                            }))
                        })
                    }))
                    .prop_signal("error", state.persona_error.signal())
                }),
                html!("input-wrapper", {
                    .prop("slot", "organization")
                    .prop("label", STR_ORGANIZATION_LABEL)
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
                    .prop("slot", "checkbox")
                    .child(
                        html!("input-checkbox", {
                            .prop_signal("error", state.terms_error_str())
                            .child(html!("div", {
                                .prop("slot", "label")
                                .text(STR_TERMS_LABEL_ONE)
                                .child(html!("a", {
                                    .attr("href", "https://www.jewishinteractive.org/terms-and-conditions/")
                                    .text(STR_TERMS_TERMS)
                                    .attr("target", "_blank")
                                    .style("color", "var(--main-blue)")
                                }))
                                .child(html!("span", {
                                    .text(STR_TERMS_LABEL_TWO)
                                }))
                                .child(html!("a", {
                                    .attr("target", "_blank")
                                    .attr("href", "https://www.jewishinteractive.org/privacy-policy/")
                                    .text(STR_TERMS_PRIVACY)
                                    .style("color", "var(--main-blue)")
                                }))
                                .text(". *")
                            }))
                            .event(clone!(state => move |evt:events::CustomToggle| {
                                *state.terms.borrow_mut() = evt.value();
                                state.evaluate_terms_error();
                            }))
                        })
                    )
                }),
                html!("input-checkbox", {
                    .prop("slot", "checkbox")
                    .prop("label", STR_MARKETING_LABEL)
                    .event(clone!(state => move |evt:events::CustomToggle| {
                        *state.marketing.borrow_mut() = evt.value();
                    }))
                }),
                html!("p", {
                    .prop("slot", "committed-to-privacy")
                    .text(STR_PROTECTING_PRIVACY)
                }),
                html!("button-rect-icon", {
                    .prop("slot", "submit")
                    .prop("color", "red")
                    .prop("size", "regular")
                    .prop("iconAfter", "arrow")
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
