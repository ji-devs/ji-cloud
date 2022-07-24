use std::rc::Rc;

use dominator::{clone, html, with_node, DomBuilder};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use utils::{
    events,
    languages::{Language, EMAIL_LANGUAGES},
    unwrap::UnwrapJiExt,
};
use web_sys::{HtmlInputElement, ShadowRoot};

use crate::member_details::component::Component;

use super::EditAbout;

pub const STR_LOCATION: &str = "Location";
pub const STR_ORGANIZATION: &str = "Organization";
pub const STR_PERSONA: &str = "Persona";
pub const STR_LANGUAGE: &str = "Language";
pub const STR_PERSONA_OPTIONS: &[&str] = &[
    "Teacher",
    "Parent",
    "Therapist",
    "Grandparent",
    "School leader",
    "Administrator",
    "Shaliach",
    "Tutor",
    "Content manager",
];

impl Component for Rc<EditAbout> {
    fn styles() -> &'static str {
        r#"
            :host {
                background: white;
                padding: 30px;
                border-radius: 16px;
                box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
            }
        "#
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom.children(&mut [
            html!("input-wrapper", {
                .property("slot", "location")
                .property("label", STR_LOCATION)
                .child(html!("input-location", {
                    .property_signal("locationAsString", state.location.signal_cloned().map(|location| {
                        location.unwrap_or_default()
                            .as_str()
                            .unwrap_or_default()
                            .to_owned()
                    }))
                    .event(clone!(state => move |evt: events::GoogleLocation| {
                        let raw = serde_json::to_value(evt.raw_json()).unwrap_ji();
                        state.location.set(Some(raw));
                    }))
                }))
                .child(html!("img-ui", {
                    .property("slot", "icon")
                    .property("path", "core/inputs/pencil-blue-darker.svg")
                }))
            }),
            html!("input-checkbox", {
                .property("type", "checkbox")
                .property_signal("checked", state.location_public.signal())
                .event(clone!(state => move |evt: events::CustomToggle| {
                    state.location_public.set_neq(evt.value());
                }))
            }),
            html!("hr"),
            html!("input-wrapper", {
                .property("slot", "organization")
                .property("label", STR_ORGANIZATION)
                .child(html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .property_signal("value", state.organization.signal_cloned().map(|i| i.unwrap_or_default()))
                        .event(clone!(state => move |_: events::Input| {
                            state.organization.set(Some(elem.value()));
                        }))
                    })
                }))
                .child(html!("img-ui", {
                    .property("slot", "icon")
                    .property("path", "core/inputs/pencil-blue-darker.svg")
                }))
            }),

            html!("input-checkbox", {
                .property("type", "checkbox")
                .property_signal("checked", state.organization_public.signal())
                .event(clone!(state => move |evt: events::CustomToggle| {
                    state.organization_public.set_neq(evt.value());
                }))
            }),
            html!("hr"),

            html!("input-select", {
                .property("slot", "persona")
                .property("label", STR_PERSONA)
                .property("multiple", true)
                .property_signal("value", state.persona.signal_vec_cloned().to_signal_cloned().map(|persona| {
                    persona.join(", ")
                }))
                .children(STR_PERSONA_OPTIONS.iter().map(|persona| {
                    html!("input-select-option", {
                        .text(persona)
                        .property_signal(
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

            html!("input-checkbox", {
                .property("type", "checkbox")
                .property_signal("checked", state.persona_public.signal())
                .event(clone!(state => move |evt: events::CustomToggle| {
                    state.persona_public.set_neq(evt.value());
                }))
            }),
            html!("hr"),
            html!("input-select", {
                .property("slot", "language")
                .property("label", STR_LANGUAGE)
                .property_signal("value", state.language.signal_cloned().map(|code| {
                    Language::code_to_display_name(&code)
                }))
                .children(EMAIL_LANGUAGES.iter().map(|lang| {
                    html!("input-select-option", {
                        .text(lang.display_name())
                        .event(clone!(state => move |_: events::CustomSelectedChange| {
                            state.language.set(lang.code().to_string());
                        }))
                    })
                }))
            }),
            html!("input-checkbox", {
                .property("type", "checkbox")
                .property_signal("checked", state.language_public.signal())
                .event(clone!(state => move |evt: events::CustomToggle| {
                    state.language_public.set_neq(evt.value());
                }))
            }),
            html!("hr"),

            html!("button-rect", {
                .text("Apply")
                .property("slot", "submit")
                .event(clone!(state => move |_: events::Click| {
                    let user = state.get_user_profile_from_fields();
                    (state.callbacks.save_changes)(user);
                }))
            }),
            html!("fa-button", {
                .property("slot", "close")
                .property("icon", "fa-regular fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    (state.callbacks.close)();
                }))
            }),
        ])
    }
}
