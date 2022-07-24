use std::rc::Rc;

use components::{
    dialog::Dialog,
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::{asset::DraftOrLive, jig::JigResponse, user::public_user::PublicUser};
use utils::{
    events,
    jig::{JigPlayerOptions, ResourceContentExt},
    prelude::{get_user_cloned, get_user_id},
    routes::{CommunityMembersRoute, CommunityRoute, Route},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

use super::{
    callbacks::EditProfileCallbacks, component::Component, edit_about::EditAbout,
    edit_bio::EditBio, edit_image::EditImage, ActivePopup, Connections, Creations, MemberDetails,
};

const STR_FOLLOWING: &str = "Following";
const STR_FOLLOW: &str = "Follow";

impl MemberDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_data();

        let is_current_user = match get_user_id() {
            Some(user_id) => user_id == state.member_id,
            None => false,
        };

        html!("div", {
            .child_signal(state.member.signal_ref(clone!(state => move |member| {
                member.as_ref().map(|member| {
                    html!("community-member-details", {
                        .property("givenName", &member.given_name)
                        .property("familyName", &member.family_name)
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
                            if let Some(persona) = &member.persona {
                                dom = dom.property("persona", persona.join(", "));
                            }
                            if let Some(bio) = &member.bio {
                                dom = dom.property("bio", bio)
                            }
                            dom
                        })
                        .child(html!("profile-image", {
                            .property("slot", "profile-image")
                            .property("imageId", {
                                match &member.profile_image {
                                    Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                                    None => JsValue::UNDEFINED,
                                }
                            })
                        }))
                        .child_signal(state.follow_button_signal())
                        .apply(|dom| {
                            state.creations_mixin(dom)
                        })
                        .apply(|dom| {
                            state.connections_mixin(dom)
                        })
                        .apply_if(is_current_user, clone!(state => move |dom| {
                            dom.children(&mut [
                                html!("fa-button", {
                                    .property("slot", "edit-profile-image")
                                    .property("icon", "fa-light fa-pen")
                                    .text("Image")
                                    .event(clone!(state => move |_: events::Click| {
                                        state.active_popup.set(Some(ActivePopup::Image))
                                    }))
                                }),
                                html!("fa-button", {
                                    .property("slot", "edit-about")
                                    .property("icon", "fa-light fa-pen")
                                    .text("about")
                                    .event(clone!(state => move |_: events::Click| {
                                        state.active_popup.set(Some(ActivePopup::About))
                                    }))
                                }),
                                html!("fa-button", {
                                    .property("slot", "edit-bio")
                                    .property("icon", "fa-light fa-pen")
                                    .text("Bio")
                                    .event(clone!(state => move |_: events::Click| {
                                        state.active_popup.set(Some(ActivePopup::Bio))
                                    }))
                                }),
                            ])
                        }))
                    })
                })
            })))
            .child_signal(state.active_popup.signal().map(clone!(state => move |active_popup| {
                active_popup.map(clone!(state => move |active_popup| {
                    Dialog::render(
                        clone!(state => move || {
                            let callbacks = EditProfileCallbacks {
                                save_changes: Box::new(clone!(state => move|user| {
                                    state.save_profile_changes(user);
                                })),
                                close: Box::new(clone!(state => move || {
                                    state.active_popup.set(None);
                                }))
                            };
                            match active_popup {
                                ActivePopup::About => {
                                    EditAbout::new(
                                        get_user_cloned().unwrap_ji(),
                                        callbacks
                                    ).render()
                                },
                                ActivePopup::Bio => {
                                    EditBio::new(
                                        get_user_cloned().unwrap_ji(),
                                        callbacks
                                    ).render()
                                },
                                ActivePopup::Image => {
                                    EditImage::new(
                                        get_user_cloned().unwrap_ji(),
                                        callbacks
                                    ).render()
                                },
                            }
                        }),
                        Some(Box::new(clone!(state => move || {
                            state.active_popup.set(None);
                        })))
                    )
                }))
            })))
            .child_signal(state.play_jig.signal_cloned().map(clone!(state => move|play_jig| {
                play_jig.map(|jig_id| {
                    let close = clone!(state => move || {
                        state.play_jig.set(None);
                    });
                    PlayerPopup::new(
                        jig_id.into(),
                        JigPlayerOptions::default(),
                        PreviewPopupCallbacks::new(close)
                    ).render(None)
                })
            })))
        })
    }

    fn follow_button_signal(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = self;
        state
            .community_state
            .followings
            .signal_ref(clone!(state => move |users_followings| {
                let is_following = match users_followings {
                    None => false,
                    Some(users_followings) => {
                        users_followings.iter().any(|followee| followee == &state.member_id)
                    },
                };
                Some(match is_following {
                    true => {
                        html!("button-rect", {
                            .property("slot", "follow")
                            .property("kind", "outline")
                            .property("size", "small")
                            .property("color", "green")
                            .child(html!("fa-icon", {
                                .property("icon", "fa-solid fa-check")
                            }))
                            .text(STR_FOLLOWING)
                            .event(clone!(state => move |_: events::Click| {
                                state.unfollow_member();
                            }))
                        })
                    },
                    false => {
                        html!("button-rect", {
                            .property("slot", "follow")
                            .property("kind", "outline")
                            .property("size", "small")
                            .property("color", "blue")
                            .text(STR_FOLLOW)
                            .event(clone!(state => move |_: events::Click| {
                                state.follow_member();
                            }))
                        })
                    },
                })
            }))
    }

    fn creations_mixin(self: &Rc<Self>, dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        let state = self;
        dom.children(&mut [
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
        .children_signal_vec(
            state
                .creations
                .signal_ref(clone!(state => move |creations| {
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
                                    state.render_resource(resources)
                                })).collect()
                            }
                        },
                        Creations::Jigs(None) | Creations::Resources(None) => vec![
                            html!("progress", {
                                .property("slot", "creation-assets")
                            })
                        ]
                    }
                }))
                .to_signal_vec(),
        )
    }

    fn render_jig(self: &Rc<Self>, jig: &JigResponse) -> Dom {
        let state = self;
        let jig_id = jig.id;
        html!("community-asset", {
            .child(ModuleThumbnail::new(
                jig.id.into(),
                jig.jig_data.modules.get(0).cloned(),
                ThumbnailFallback::Asset,
                DraftOrLive::Live,
            ).render(Some("thumbnail")))
            .property("slot", "creation-assets")
            .property("name", &jig.jig_data.display_name)
            .event(clone!(state => move |_:events::Click| {
                state.play_jig.set(Some(jig_id));
            }))
        })
    }

    fn render_resource(self: &Rc<Self>, jig: &JigResponse) -> Dom {
        let link = match jig.jig_data.additional_resources.first() {
            Some(resource) => resource.resource_content.get_link(),
            None => {
                // should not be here
                String::new()
            }
        };

        html!("community-asset", {
            .child(ModuleThumbnail::new(
                jig.id.into(),
                jig.jig_data.modules.get(0).cloned(),
                ThumbnailFallback::Asset,
                DraftOrLive::Live,
            ).render(Some("thumbnail")))
            .property("slot", "creation-assets")
            .property("name", &jig.jig_data.display_name)
            .property("href", link)
        })
    }

    fn connections_mixin(self: &Rc<Self>, dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        let state = self;
        dom.children(&mut [
            html!("community-member-details-tab", {
                .property("slot", "connection-tabs")
                .text("Followers")
                .property_signal("active", state.connections.signal_ref(|connections| {
                    matches!(connections, Connections::Followers(_))
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.set_active_connections(Connections::Followers(None));
                }))
            }),
            html!("community-member-details-tab", {
                .property("slot", "connection-tabs")
                .text("Following")
                .property_signal("active", state.connections.signal_ref(|connections| {
                    matches!(connections, Connections::Following(_))
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.set_active_connections(Connections::Following(None));
                }))
            }),
        ])
        .children_signal_vec(
            state
                .connections
                .signal_ref(clone!(state => move |connections| {
                    match connections {
                        Connections::Followers(Some(members)) => {
                            if members.is_empty() {
                                vec![
                                    html!("div", {
                                        .property("slot", "connection-members")
                                        .text("User has no followers")
                                    })
                                ]
                            } else {
                                members.iter().map(clone!(state => move |follower| {
                                    state.render_member(follower)
                                })).collect()
                            }
                        },
                        Connections::Following(Some(members)) => {
                            if members.is_empty() {
                                vec![
                                    html!("div", {
                                        .property("slot", "connection-members")
                                        .text("User is not following anyone")
                                    })
                                ]
                            } else {
                                members.iter().map(clone!(state => move |member| {
                                    state.render_member(member)
                                })).collect()
                            }
                        },
                        Connections::Followers(None) | Connections::Following(None) => vec![
                            html!("progress", {
                                .property("slot", "connection-members")
                            })
                        ]
                    }
                }))
                .to_signal_vec(),
        )
    }

    fn render_member(self: &Rc<Self>, member: &PublicUser) -> Dom {
        html!("community-member-details-connection", {
            .property("slot", "connection-members")
            .property("name", &member.given_name)
            .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(member.id))).to_string()
            }))
            .child(html!("profile-image", {
                .property("slot", "profile-image")
                .property("imageId", {
                    match &member.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
            }))
        })
    }
}
