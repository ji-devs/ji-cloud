use std::rc::Rc;

use dominator::{html, Dom};
use wasm_bindgen::JsValue;

use super::{Creations, MemberDetails};

impl MemberDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_data();

        html!("div", {
            .child_signal(state.member.signal_ref(move |member| {
                member.as_ref().map(|member| {
                    html!("div", {
                        .style("border", "dashed 1px blue")
                        .style("padding", "10px")
                        .style("margin", "10px")
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
                            html!("p", {
                                .text("bio: ")
                                .text(&member.bio)
                            }),
                        ])
                        .apply(|mut dom| {
                            if let Some(language) = &member.language {
                                dom = dom.child(html!("p", {
                                    .text("language: ")
                                    .text(language)
                                }))
                            }
                            dom
                        })
                        .apply(|mut dom| {
                            if let Some(organization) = &member.organization {
                                dom = dom.child(html!("p", {
                                    .text("organization: ")
                                    .text(organization)
                                }))
                            }
                            dom
                        })
                        .children(member.persona.iter().map(|persona| {
                            html!("p", {
                                .text("persona: ")
                                .text(&persona)
                            })
                        }))
                    })
                })
            }))
            .child_signal(state.creations.signal_ref(|creations| {
                Some(match creations {
                    Creations::Jigs(Some(jigs)) => {
                        html!("div", {
                            .children(jigs.iter().map(|jig| {
                                html!("p", {
                                    .text(&jig.jig_data.display_name)
                                })
                            }))
                        })
                    },
                    Creations::Jigs(None) => html!("div", {
                        .text("loading")
                    })
                })
            }))
        })
    }
}

// location: Option<serde_json::Value>,
// badges: Vec<BadgeId>,
