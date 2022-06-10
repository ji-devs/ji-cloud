use std::rc::Rc;

use components::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::{asset::DraftOrLive, jig::JigResponse};
use utils::events;
use wasm_bindgen::JsValue;

use super::{Creations, MemberDetails};

impl MemberDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_data();

        html!("div", {
            .child_signal(state.member.signal_ref(clone!(state => move |member| {
                member.as_ref().map(|member| {
                    html!("community-member-details", {
                        .property("bio", &member.bio)
                        .apply(|mut dom| {
                            if let Some(_location) = &member.location {
                                // add city
                                // dom = dom.property("city", city)
                            }
                            if let Some(language) = &member.language {
                                dom = dom.property("language", language)
                            }
                            if let Some(organization) = &member.organization {
                                dom = dom.property("organization", organization)
                            }
                            if !member.persona.is_empty() {
                                dom = dom.property("persona", member.persona.join(", "));
                            }
                            dom
                        })
                        .children(&mut [
                            html!("profile-image", {
                                .style("height", "40px")
                                .style("width", "40px")
                                .property("imageId", {
                                    match &member.profile_image {
                                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                                        None => JsValue::UNDEFINED,
                                    }
                                })
                            }),
                            html!("p", {
                                .text("username: ")
                                .text(&member.username)
                            }),
                            html!("p", {
                                .text("given_name: ")
                                .text(&member.given_name)
                            }),
                            html!("p", {
                                .text("family_name: ")
                                .text(&member.family_name)
                            }),
                        ])
                        .children(&mut [
                            html!("community-member-details-tab", {
                                .property("slot", "creation-tabs")
                                .text("JIGs")
                                .property_signal("active", state.creations.signal_ref(|creations| {
                                    matches!(creations, Creations::Jigs(_))
                                }))
                                .event(clone!(state => move |_: events::Click| {
                                    state.set_active_creations(Creations::Jigs(None));
                                }))
                            }),
                            html!("community-member-details-tab", {
                                .property("slot", "creation-tabs")
                                .text("Resources")
                                .property_signal("active", state.creations.signal_ref(|creations| {
                                    matches!(creations, Creations::Resources(_))
                                }))
                                .event(clone!(state => move |_: events::Click| {
                                    state.set_active_creations(Creations::Resources(None));
                                }))
                            }),
                        ])
                        .children_signal_vec(state.creations.signal_ref(clone!(state => move |creations| {
                            match creations {
                                Creations::Jigs(Some(jigs)) => {
                                    if jigs.is_empty() {
                                        vec![
                                            html!("div", {
                                                .property("slot", "creation-assets")
                                                .text("User has no JIGs")
                                            })
                                        ]
                                    } else {
                                        jigs.iter().map(clone!(state => move |jig| {
                                            state.render_jig(jig)
                                        })).collect()
                                    }
                                },
                                Creations::Resources(Some(resources)) => {
                                    if resources.is_empty() {
                                        vec![
                                            html!("div", {
                                                .property("slot", "creation-assets")
                                                .text("User has no resources")
                                            })
                                        ]
                                    } else {
                                        resources.iter().map(clone!(state => move |resources| {
                                            state.render_jig(resources)
                                        })).collect()
                                    }
                                },
                                Creations::Jigs(None) | Creations::Resources(None) => vec![
                                    html!("progress", {
                                        .property("slot", "creation-assets")
                                    })
                                ]
                            }
                        })).to_signal_vec())
                    })
                })
            })))
        })
    }

    fn render_jig(self: &Rc<Self>, jig: &JigResponse) -> Dom {
        html!("community-asset", {
            .child(ModuleThumbnail::new(
                jig.id.into(),
                jig.jig_data.modules.get(0).cloned(),
                ThumbnailFallback::Asset,
                DraftOrLive::Live,
            ).render(Some("thumbnail")))
            .property("slot", "creation-assets")
            .property("name", &jig.jig_data.display_name)
        })
    }
}

// location: Option<serde_json::Value>,
// badges: Vec<BadgeId>,
