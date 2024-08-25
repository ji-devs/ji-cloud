use std::rc::Rc;

use super::EditAbout;
use dominator::{clone, html, with_node, DomBuilder};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use itertools::Itertools;
use utils::{
    component::Component,
    events,
    languages::{Language, JIG_LANGUAGES},
};
use web_sys::{HtmlInputElement, ShadowRoot};

pub const STR_HEADING: &str = "Edit your details";
// pub const STR_LOCATION: &str = "Where I live";
pub const STR_ORGANIZATION: &str = "Where I work";
pub const STR_PERSONA: &str = "What I do";
pub const STR_LANGUAGE: &str = "Language";
pub const STR_PERSONA_OPTIONS: &[&str] = &[
    "Teacher",
    "Student teacher",
    "Parent",
    "Therapist",
    "Grandparent",
    "School leader",
    "Administrator",
    "Shaliach",
    "Tutor",
    "Content manager",
];

impl Component<EditAbout> for Rc<EditAbout> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child(html!("popup-body", {
            .child(html!("fa-button", {
                .prop("slot", "close")
                .prop("icon", "fa-regular fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    (state.callbacks.close)();
                }))
            }))
            .child(html!("h3", {
                .prop("slot", "heading")
                .text(STR_HEADING)
            }))
            .child(html!("div", {
                .prop("slot", "body")
                .class("field-grid")
                .children(&mut [
                    // TODO: Temporary disabled location to prevent usage of Places API
                    // html!("fa-icon", {
                    //     .prop("icon", "fa-solid fa-location-dot")
                    // }),
                    // html!("input-wrapper", {
                    //     .prop("slot", "location")
                    //     .prop("label", STR_LOCATION)
                    //     .child(html!("input-location", {
                    //         .prop_signal("locationAsString", state.location.signal_cloned().map(|location| {
                    //             location.unwrap_or_default()
                    //                 .as_str()
                    //                 .unwrap_or_default()
                    //                 .to_owned()
                    //         }))
                    //         .event(clone!(state => move |evt: events::GoogleLocation| {
                    //             let raw = serde_json::to_value(evt.raw_json()).unwrap_ji();
                    //             state.location.set(Some(raw));
                    //         }))
                    //     }))
                    //     .child(html!("img-ui", {
                    //         .prop("slot", "icon")
                    //         .prop("path", "core/inputs/pencil-blue-darker.svg")
                    //     }))
                    // }),
                    // html!("community-private-public-switch", {
                    //     .prop("type", "checkbox")
                    //     .prop_signal("isPublic", state.location_public.signal())
                    //     .event(clone!(state => move |evt: events::CustomToggle| {
                    //         state.location_public.set_neq(evt.value());
                    //     }))
                    // }),
                    html!("fa-icon", {
                        .prop("icon", "fa-solid fa-briefcase")
                    }),
                    html!("input-wrapper", {
                        .prop("slot", "organization")
                        .prop("label", STR_ORGANIZATION)
                        .child(html!("input" => HtmlInputElement, {
                            .with_node!(elem => {
                                .prop_signal("value", state.organization.signal_cloned().map(|i| i.unwrap_or_default()))
                                .event(clone!(state => move |_: events::Input| {
                                    state.organization.set(Some(elem.value()));
                                }))
                            })
                        }))
                        .child(html!("img-ui", {
                            .prop("slot", "icon")
                            .prop("path", "core/inputs/pencil-blue-darker.svg")
                        }))
                    }),
                    html!("community-private-public-switch", {
                        .prop("type", "checkbox")
                        .prop_signal("isPublic", state.organization_public.signal())
                        .event(clone!(state => move |evt: events::CustomToggle| {
                            state.organization_public.set_neq(evt.value());
                        }))
                    }),
                    html!("fa-icon", {
                        .prop("icon", "fa-regular fa-id-card-clip")
                    }),
                    html!("input-select", {
                        .prop("slot", "persona")
                        .prop("label", STR_PERSONA)
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
                                }))
                            })
                        }))
                    }),
                    html!("community-private-public-switch", {
                        .prop("type", "checkbox")
                        .prop_signal("isPublic", state.persona_public.signal())
                        .event(clone!(state => move |evt: events::CustomToggle| {
                            state.persona_public.set_neq(evt.value());
                        }))
                    }),
                    html!("fa-icon", {
                        .prop("icon", "fa-solid fa-globe")
                    }),
                    html!("input-select", {
                        .prop("slot", "language")
                        .prop("label", STR_LANGUAGE)
                        .prop("multiple", true)
                        .prop_signal("value", state.languages_spoken.signal_ref(|languages_spoken| {
                            languages_spoken.iter().map(|l| Language::code_to_display_name(l)).join(", ")
                        }))
                        .children(JIG_LANGUAGES.iter().map(|lang| {
                            html!("input-select-option", {
                                .text(lang.display_name())
                                .prop_signal("selected", state.languages_spoken.signal_cloned().map(clone!(lang => move |languages_spoken| {
                                    languages_spoken.contains(lang.code())
                                })))
                                .event(clone!(state => move |_: events::CustomSelectedChange| {
                                    let mut languages_spoken = state.languages_spoken.lock_mut();
                                    let lang = lang.code().to_string();
                                    if languages_spoken.contains(&lang) {
                                        languages_spoken.remove(&lang);
                                    } else {
                                        languages_spoken.insert(lang);
                                    }
                                }))
                            })
                        }))
                    }),
                    html!("community-private-public-switch", {
                        .prop("type", "checkbox")
                        .prop_signal("isPublic", state.languages_spoken_public.signal())
                        .event(clone!(state => move |evt: events::CustomToggle| {
                            state.languages_spoken_public.set_neq(evt.value());
                        }))
                    }),
                ])
            }))
            .child(html!("button-rect", {
                .text("Save")
                .prop("slot", "body")
                .event(clone!(state => move |_: events::Click| {
                    let user = state.get_user_profile_from_fields();
                    (state.callbacks.save_changes)(user);
                }))
            }))
        }))
    }
}
