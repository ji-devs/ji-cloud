use std::rc::Rc;

use super::{
    callbacks::EditProfileCallbacks, edit_about::EditAbout, edit_bio::EditBio,
    edit_image::EditImage, ActivePopup, Connections, Creations, MemberDetails,
};
use components::{
    asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig},
    dialog::Dialog,
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, link, Dom, DomBuilder};
use futures_signals::signal::{Signal, SignalExt};
use itertools::Itertools;
use shared::{
    domain::{asset::Asset, user::public_user::PublicUser},
    media::MediaLibrary,
};
use utils::{
    component::Component,
    events,
    languages::Language,
    prelude::{get_user_cloned, get_user_id},
    routes::{CommunityCirclesRoute, CommunityMembersRoute, CommunityRoute, Route},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

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
                        .prop("givenName", &member.given_name)
                        .prop("familyName", &member.family_name)
                        .apply(|mut dom| {
                            if let Some(_location) = &member.location {
                                // add city
                                // dom = dom.prop("city", city)
                            }
                            if let Some(languages_spoken) = &member.languages_spoken {
                                if languages_spoken.len() > 0 {
                                    let languages = languages_spoken.iter().map(|l| Language::code_to_display_name(l)).join(", ");
                                    dom = dom.prop("language", languages)
                                }
                            }
                            if let Some(organization) = &member.organization {
                                dom = dom.prop("organization", organization)
                            }
                            if let Some(persona) = &member.persona {
                                dom = dom.prop("persona", persona.join(", "));
                            }
                            if let Some(bio) = &member.bio {
                                dom = dom.prop("bio", bio)
                            }
                            dom
                        })
                        .child(html!("profile-image", {
                            .prop("slot", "profile-image")
                            .prop("imageId", {
                                match &member.profile_image {
                                    Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                                    None => JsValue::UNDEFINED,
                                }
                            })
                        }))
                        .children_signal_vec(state.circles.signal_ref(move |circles| {
                            circles.iter().map(move |circle| {
                                link!(Route::Community(CommunityRoute::Circles(CommunityCirclesRoute::Circle(circle.id))).to_string(), {
                                    .prop("slot", "circles")
                                    .prop("title", &circle.display_name)
                                    .child(html!("img-ji", {
                                        .style("height", "70px")
                                        .style("width", "70px")
                                        .style("box-shadow", "0 0 8px 0 rgba(0, 0, 0, 0.06)")
                                        .style("border", "solid 1px var(--light-gray-1)")
                                        .style("border-radius", "50%")
                                        .style("overflow", "hidden")
                                        .prop("lib", MediaLibrary::User.to_str())
                                        .prop("id", &circle.image.0.to_string())
                                    }))
                                    .child(html!("span", {
                                        .style("white-space", "nowrap")
                                        .style("overflow", "hidden")
                                        .style("text-overflow", "ellipsis")
                                        .style("max-width", "100%")
                                        .text(&circle.display_name)
                                    }))
                                })
                            }).collect()
                        }).to_signal_vec())
                        .apply_if(!is_current_user, clone!(state => move |dom| {
                            dom.child_signal(state.follow_button_signal())
                        }))
                        .apply(|dom| {
                            state.creations_mixin(dom)
                        })
                        .apply(|dom| {
                            state.connections_mixin(dom)
                        })
                        .apply_if(is_current_user, clone!(state => move |dom| {
                            dom.children(&mut [
                                html!("fa-button", {
                                    .prop("slot", "edit-profile-image")
                                    .prop("icon", "fa-light fa-pen")
                                    .text("Image")
                                    .event(clone!(state => move |_: events::Click| {
                                        state.active_popup.set(Some(ActivePopup::Image))
                                    }))
                                }),
                                html!("fa-button", {
                                    .prop("slot", "edit-about")
                                    .prop("icon", "fa-light fa-pen")
                                    .text("about")
                                    .event(clone!(state => move |_: events::Click| {
                                        state.active_popup.set(Some(ActivePopup::About))
                                    }))
                                }),
                                html!("fa-button", {
                                    .prop("slot", "edit-bio")
                                    .prop("icon", "fa-light fa-pen")
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
            .child_signal(state.play_asset.signal_cloned().map(clone!(state => move|play_asset| {
                play_asset.map(|jig_id| {
                    let close = clone!(state => move || {
                        state.play_asset.set(None);
                    });
                    PlayerPopup::new_default_player_options(
                        jig_id.into(),
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
                            .prop("slot", "follow")
                            .prop("kind", "outline")
                            .prop("size", "small")
                            .prop("color", "green")
                            .child(html!("fa-icon", {
                                .prop("icon", "fa-solid fa-check")
                            }))
                            .text(STR_FOLLOWING)
                            .event(clone!(state => move |_: events::Click| {
                                state.unfollow_member();
                            }))
                        })
                    },
                    false => {
                        html!("button-rect", {
                            .prop("slot", "follow")
                            .prop("kind", "outline")
                            .prop("size", "small")
                            .prop("color", "blue")
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
                .prop("slot", "creation-tabs")
                .text("JIGs")
                .prop_signal("active", state.creations.signal_ref(|creations| {
                    matches!(creations, Creations::Jigs(_))
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.set_active_creations(Creations::Jigs(None));
                }))
            }),
            html!("community-member-details-tab", {
                .prop("slot", "creation-tabs")
                .text("Playlists")
                .prop_signal("active", state.creations.signal_ref(|creations| {
                    matches!(creations, Creations::Playlists(_))
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.set_active_creations(Creations::Playlists(None));
                }))
            }),
            html!("community-member-details-tab", {
                .prop("slot", "creation-tabs")
                .text("Resources")
                .prop_signal("active", state.creations.signal_ref(|creations| {
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
                                        .prop("slot", "creation-assets")
                                        .text("User has no JIGs")
                                    })
                                ]
                            } else {
                                jigs.iter().map(clone!(state => move |jig| {
                                    state.render_asset(jig.clone().into())
                                })).collect()
                            }
                        },
                        Creations::Playlists(Some(playlists)) => {
                            if playlists.is_empty() {
                                vec![
                                    html!("div", {
                                        .prop("slot", "creation-assets")
                                        .text("User has no playlists")
                                    })
                                ]
                            } else {
                                playlists.iter().map(clone!(state => move |playlists| {
                                    state.render_asset(playlists.clone().into())
                                })).collect()
                            }
                        },
                        Creations::Resources(Some(resources)) => {
                            if resources.is_empty() {
                                vec![
                                    html!("div", {
                                        .prop("slot", "creation-assets")
                                        .text("User has no resources")
                                    })
                                ]
                            } else {
                                resources.iter().map(clone!(state => move |resources| {
                                    state.render_asset(resources.clone().into())
                                })).collect()
                            }
                        },
                        Creations::Jigs(None) | Creations::Playlists(None) | Creations::Resources(None) => vec![
                            html!("progress", {
                                .prop("slot", "creation-assets")
                            })
                        ]
                    }
                }))
                .to_signal_vec(),
        )
    }

    fn render_asset(self: &Rc<Self>, asset: Asset) -> Dom {
        let state = self;
        let asset_id = asset.id();
        render_asset_card(
            &asset,
            AssetCardConfig {
                bottom_indicator: AssetCardBottomIndicator::Author,
                slot: Some("creation-assets"),
                dense: true,
                menu: Some(Rc::new(clone!(state => move || {
                    html!("menu-kebab", {
                        .prop("slot", "menu")
                        .children(&mut [
                            html!("menu-line", {
                                .prop("icon", "play")
                                .event(clone!(state => move |_: events::Click| {
                                    state.play_asset.set(Some(asset_id));
                                }))
                            })
                        ])
                    })
                }))),
            },
        )
    }

    fn connections_mixin(self: &Rc<Self>, dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        let state = self;
        dom.children(&mut [
            html!("community-member-details-tab", {
                .prop("slot", "connection-tabs")
                .text("Followers")
                .prop_signal("active", state.connections.signal_ref(|connections| {
                    matches!(connections, Connections::Followers(_))
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.set_active_connections(Connections::Followers(None));
                }))
            }),
            html!("community-member-details-tab", {
                .prop("slot", "connection-tabs")
                .text("Following")
                .prop_signal("active", state.connections.signal_ref(|connections| {
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
                                        .prop("slot", "connection-members")
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
                                        .prop("slot", "connection-members")
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
                                .prop("slot", "connection-members")
                            })
                        ]
                    }
                }))
                .to_signal_vec(),
        )
    }

    fn render_member(self: &Rc<Self>, member: &PublicUser) -> Dom {
        html!("community-member-details-connection", {
            .prop("slot", "connection-members")
            .prop("name", &format!("{} {}", member.given_name, member.family_name))
            .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(member.id))).to_string()
            }))
            .child(html!("profile-image", {
                .prop("slot", "profile-image")
                .prop("imageId", {
                    match &member.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
            }))
        })
    }
}
