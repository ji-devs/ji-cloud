use std::rc::Rc;

use crate::{circle_card::CircleCard, member_card::MemberCard};

use super::{
    callbacks::EditProfileCallbacks, edit_about::EditAbout, edit_bio::EditBio,
    edit_image::EditImage, ActivePopup, MemberDetails,
};
use components::{
    asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::SignalExt;
use itertools::Itertools;
use shared::domain::{asset::Asset, user::public_user::PublicUser};
use utils::{
    asset::ResourceContentExt,
    component::Component,
    dialog, events,
    languages::Language,
    prelude::{get_user_cloned, get_user_id},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlElement, ScrollBehavior, ScrollIntoViewOptions, ShadowRoot};

const STR_FOLLOWING: &str = "Following";
const STR_FOLLOW: &str = "Follow";

impl Component<MemberDetails> for Rc<MemberDetails> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        state.load_data();

        let is_current_user = match get_user_id() {
            Some(user_id) => user_id == state.member_id,
            None => false,
        };

        dom.child_signal(state.member.signal_ref(clone!(state => move |member| {
            member.as_ref().map(|member| {
                state.render_header(member, is_current_user)
            })
        })))
        .child_signal(state.member.signal_ref(clone!(state => move |member| {
            member.as_ref().map(|member| {
                state.render_about(&member, is_current_user)
            })
        })))
        .child(state.render_creations())
        .child(state.render_network())
        .apply(|dom| state.popups_mixin(dom))
    }
}

impl MemberDetails {
    fn render_header(self: &Rc<Self>, member: &PublicUser, is_current_user: bool) -> Dom {
        let state = self;

        html!("div", {
            .class("header")
            .child(html!("profile-image", {
                .prop("imageId", {
                    match &member.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
                .prop("givenName", &member.given_name)
                .prop("familyName", &member.family_name)
            }))
            .apply_if(is_current_user, clone!(state => move |dom| {
                dom.children(&mut [
                    html!("fa-button", {
                        .class("edit-image")
                        .prop("icon", "fa-light fa-pen")
                        .text("Image")
                        .event(clone!(state => move |_: events::Click| {
                            state.active_popup.set(Some(ActivePopup::Image))
                        }))
                    }),
                ])
            }))
            .child(html!("span", {
                .class("name")
                .text(&member.given_name)
                .text(" ")
                .text(&member.family_name)
            }))
            .child(html!("span", {
                .class("badge")
                .apply(|mut dom| {
                    if let Some(badge) = member.badge {
                        dom = dom.text(badge.display_name());
                    }
                    dom
                })
            }))
            .apply_if(!is_current_user, clone!(state => move |dom| {
                dom.child_signal(state.is_following.signal().map(clone!(state => move |is_following| {
                    is_following.map(clone!(state => move |is_following| {
                        match is_following {
                            true => {
                                html!("button-rect", {
                                    .class("follow-button")
                                    .prop("kind", "outline")
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
                                    .class("follow-button")
                                    .prop("kind", "outline")
                                    .prop("color", "blue")
                                    .text(STR_FOLLOW)
                                    .event(clone!(state => move |_: events::Click| {
                                        state.follow_member();
                                    }))
                                })
                            },
                        }
                    }))
                })))
            }))
            .child(html!("hr"))
            .child(html!("div", {
                .class("anchors")
                .children(&mut [
                    html!("button-rect", {
                        .prop("kind", "filled")
                        .prop("color", "blue")
                        .text("Jigzi creations")
                        .event(|e: events::Click| {
                            jump_to(e.dyn_target().unwrap_ji(), "#creations");
                        })
                    }),
                    html!("button-rect", {
                        .prop("kind", "outline")
                        .prop("color", "grey")
                        .text("Network")
                        .event(|e: events::Click| {
                            jump_to(e.dyn_target().unwrap_ji(), "#network");
                        })
                    }),
                ])
            }))
        })
    }

    fn render_about(self: &Rc<Self>, member: &PublicUser, is_current_user: bool) -> Dom {
        let state = self;
        html!("section", {
            .class("about-section")

            .child(html!("div", {
                .class("header-with-edit")
                .child(html!("h3", {
                    .text("About")
                }))
                .apply_if(is_current_user, clone!(state => move |dom| {
                    dom.child(html!("fa-button", {
                        .prop("icon", "fa-light fa-pen")
                        .text("about")
                        .event(clone!(state => move |_: events::Click| {
                            state.active_popup.set(Some(ActivePopup::About))
                        }))
                    }))
                }))
            }))
            .child(html!("div", {
                .class("icon-label")
                .apply(|mut dom| {
                    if let Some(country) = &member.country_long {
                        dom = dom.child(html!("div", {
                            .class("item")
                            .child(html!("img-ui", {
                                .prop("path", "entry/community/member-details/location.svg")
                            }))
                            .child(html!("p", {
                                .text(&country)
                            }))
                        }));
                    }
                    if let Some(organization) = &member.organization {
                        dom = dom.child(html!("div", {
                            .class("item")
                            .child(html!("img-ui", {
                                .prop("path", "entry/community/member-details/organization.svg")
                            }))
                            .child(html!("p", {
                                .text(&organization)
                            }))
                        }));
                    }
                    if let Some(persona) = &member.persona {
                        dom = dom.child(html!("div", {
                            .class("item")
                            .child(html!("img-ui", {
                                .prop("path", "entry/community/member-details/persona.svg")
                            }))
                            .child(html!("p", {
                                .text(&persona.join(", "))
                            }))
                        }));
                    }
                    if let Some(languages_spoken) = &member.languages_spoken {
                        if languages_spoken.len() > 0 {
                            let languages = languages_spoken.iter().map(|l| Language::code_to_display_name(l)).join(", ");
                            dom = dom.child(html!("div", {
                                .class("item")
                                .child(html!("img-ui", {
                                    .prop("path", "entry/community/member-details/language.svg")
                                }))
                                .child(html!("p", {
                                    .text(&languages)
                                }))
                            }));
                        }
                    }
                    dom
                })
            }))
            .apply(|mut dom| {
                if member.bio.is_some() || is_current_user {
                    dom = dom
                        .child(html!("hr"))
                        .child(html!("div", {
                            .class("header-with-edit")
                            .child(html!("h3", {
                                .text("Bio")
                            }))
                            .apply_if(is_current_user, clone!(state => move |dom| {
                                dom.child(html!("fa-button", {
                                    .prop("icon", "fa-light fa-pen")
                                    .text("Bio")
                                    .event(clone!(state => move |_: events::Click| {
                                        state.active_popup.set(Some(ActivePopup::Bio))
                                    }))
                                }))
                            }))
                        }))
                }
                if let Some(bio) = &member.bio {
                    dom = dom
                        .child(html!("p", {
                            .class("about-text")
                            .attr("dir", "auto")
                            .text(&bio)
                        }))
                }
                dom
            })
        })
    }

    fn render_creations(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("section", {
            .prop("id", "creations")
            .class("creations-section")
            .child(html!("h3", {
                .text("Jigzi creations")
            }))
            .child_signal(state.jigs.signal_cloned().map(clone!(state => move|jigs| {
                if matches!(&jigs, Some(jigs) if jigs.is_empty()) {
                    return None;
                }
                Some(match jigs {
                    None => html!("progress"),
                    Some(jigs) => {
                        if jigs.is_empty() {
                            return None;
                        }
                        html!("div", {
                            .class("items-container")
                            .class("jigs")
                            .child(html!("h4", {
                                .text("Jigs")
                                .text_signal(state.jigs_count.signal().map(|count| {
                                    count.map(|count| format!(" ({})", count.to_string())).unwrap_or_default()
                                }))
                            }))
                            .child(html!("div", {
                                .class("items")
                                .children(jigs.into_iter().map(|jig| {
                                    state.render_asset(jig.into())
                                }))
                            }))
                        })
                    },
                })
            })))
            .child_signal(state.playlists.signal_cloned().map(clone!(state => move|playlists| {
                if matches!(&playlists, Some(playlists) if playlists.is_empty()) {
                    return None;
                }
                Some(match playlists {
                    None => html!("progress"),
                    Some(playlists) => {
                        if playlists.is_empty() {
                            return None;
                        }
                        html!("div", {
                            .class("items-container")
                            .class("playlists")
                            .child(html!("h4", {
                                .text("Playlists")
                                .text_signal(state.playlists_count.signal().map(|count| {
                                    count.map(|count| format!(" ({})", count.to_string())).unwrap_or_default()
                                }))
                            }))
                            .child(html!("div", {
                                .class("items")
                                .children(playlists.into_iter().map(|playlist| {
                                    state.render_asset(playlist.into())
                                }))
                            }))
                        })
                    },
                })
            })))
            .child_signal(state.resources.signal_cloned().map(clone!(state => move|resources| {
                if matches!(&resources, Some(resources) if resources.is_empty()) {
                    return None;
                }
                Some(match resources {
                    None => html!("progress"),
                    Some(resources) => {
                        if resources.is_empty() {
                            return None;
                        }
                        html!("div", {
                            .class("items-container")
                            .class("resources")
                            .child(html!("h4", {
                                .text("Resources")
                                .text_signal(state.resources_count.signal().map(|count| {
                                    count.map(|count| format!(" ({})", count.to_string())).unwrap_or_default()
                                }))
                            }))
                            .child(html!("div", {
                                .class("items")
                                .children(resources.into_iter().map(|resources| {
                                    state.render_asset(resources.into())
                                }))
                            }))
                        })
                    },
                })
            })))
            .child_signal(state.courses.signal_cloned().map(clone!(state => move|courses| {
                if matches!(&courses, Some(courses) if courses.is_empty()) {
                    return None;
                }
                Some(match courses {
                    None => html!("progress"),
                    Some(courses) => {
                        if courses.is_empty() {
                            return None;
                        }
                        html!("div", {
                            .class("items-container")
                            .class("courses")
                            .child(html!("h4", {
                                .text("Courses")
                                .text_signal(state.courses_count.signal().map(|count| {
                                    count.map(|count| format!(" ({})", count.to_string())).unwrap_or_default()
                                }))
                            }))
                            .child(html!("div", {
                                .class("items")
                                .children(courses.into_iter().map(|course| {
                                    state.render_asset(course.into())
                                }))
                            }))
                        })
                    },
                })
            })))
        })
    }

    fn render_asset(self: &Rc<Self>, asset: Asset) -> Dom {
        let state = self;
        let asset_id = asset.id();
        let is_resource = asset.is_resource();
        let additional_resource_0 = asset.additional_resources().first().cloned();
        render_asset_card(
            &asset,
            AssetCardConfig {
                bottom_indicator: AssetCardBottomIndicator::Author,
                dense: true,
                menu: Some(Rc::new(clone!(state => move || {
                    html!("menu-kebab", {
                        .prop("slot", "menu")
                        .apply_if(!is_resource, |dom| {
                            dom.child(html!("menu-line", {
                                .prop("icon", "play")
                                .event(clone!(state => move |_: events::Click| {
                                    state.play_asset.set(Some(asset_id));
                                }))
                            }))
                        })
                        .apply_if(is_resource, clone!(additional_resource_0 => move |dom| {
                            dom.child(html!("menu-line", {
                                .prop("icon", "view")
                                .event(clone!(additional_resource_0 => move |_: events::Click| {
                                    if let Some(resource) = &additional_resource_0 {
                                        let _ = window()
                                            .unwrap_ji()
                                            .open_with_url(&resource.resource_content.get_link())
                                            .unwrap_ji();
                                    }
                                }))
                            }))
                        }))
                    })
                }))),
                ..Default::default()
            },
        )
    }

    fn render_network(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("section", {
            .prop("id", "network")
            .class("network-section")
            .child(html!("h3", {
                .text_signal(state.member.signal_ref(|member| {
                    member.as_ref().map(|member| {
                        format!("{} {}'s network", member.given_name, member.family_name)
                    }).unwrap_or_default()
                }))
            }))
            .child_signal(state.circles.signal_ref(clone!(state => move|circles| {
                if matches!(&circles, Some(circles) if circles.is_empty()) {
                    return None;
                }
                Some(match circles {
                    None => html!("progress"),
                    Some(circles) => {
                        if circles.is_empty() {
                            return None;
                        }
                        html!("div", {
                            .class("items-container")
                            .class("circles")
                            .child(html!("h4", {
                                .text("Circles")
                                .text_signal(state.circles_count.signal().map(|count| {
                                    count.map(|count| format!(" ({})", count.to_string())).unwrap_or_default()
                                }))
                            }))
                            .child(html!("div", {
                                .class("items")
                                .children(circles.into_iter().map(|circle| {
                                    CircleCard {
                                        circle,
                                    }.render()
                                }))
                            }))
                        })
                    },
                })
            })))
            .child_signal(state.following.signal_ref(clone!(state => move|following| {
                let current_user_id = get_user_id();
                if matches!(&following, Some(following) if following.is_empty()) {
                    return None;
                }
                Some(match following {
                    None => html!("progress"),
                    Some(following) => {
                        if following.is_empty() {
                            return None;
                        }
                        html!("div", {
                            .class("items-container")
                            .class("following")
                            .child(html!("h4", {
                                .text("Following")
                                .text_signal(state.following_count.signal().map(|count| {
                                    count.map(|count| format!(" ({})", count.to_string())).unwrap_or_default()
                                }))
                            }))
                            .child(html!("div", {
                                .class("items")
                                .children(following.into_iter().map(|member| {
                                    MemberCard {
                                        member,
                                        menu: None,
                                        admin_tag: false,
                                        current_user_id,
                                    }.render()
                                }))
                            }))
                        })
                    },
                })
            })))
            .child_signal(state.followers.signal_ref(clone!(state => move |followers| {
                let current_user_id = get_user_id();
                if matches!(&followers, Some(followers) if followers.is_empty()) {
                    return None;
                }
                Some(match followers {
                    None => html!("progress"),
                    Some(followers) => {
                        if followers.is_empty() {
                            return None;
                        }
                        html!("div", {
                            .class("items-container")
                            .class("followers")
                            .child(html!("h4", {
                                .text("Followers")
                                .text_signal(state.followers_count.signal().map(|count| {
                                    count.map(|count| format!(" ({})", count.to_string())).unwrap_or_default()
                                }))
                            }))
                            .child(html!("div", {
                                .class("items")
                                .children(followers.into_iter().map(|member| {
                                    MemberCard {
                                        member,
                                        menu: None,
                                        admin_tag: false,
                                        current_user_id,
                                    }.render()
                                }))
                            }))
                        })
                    },
                })
            })))
        })
    }

    fn popups_mixin(self: &Rc<Self>, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child_signal(
            state
                .active_popup
                .signal()
                .map(clone!(state => move |active_popup| {
                    active_popup.map(clone!(state => move |active_popup| {
                        let callbacks = EditProfileCallbacks {
                            save_changes: Box::new(clone!(state => move|user| {
                                state.save_profile_changes(user);
                            })),
                            close: Box::new(clone!(state => move || {
                                state.active_popup.set(None);
                            }))
                        };
                        dialog!{
                            .child(match active_popup {
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
                            })
                        }
                    }))
                })),
        )
        .child_signal(state.play_asset.signal_cloned().map(
            clone!(state => move|play_asset| {
                play_asset.map(|jig_id| {
                    let close = clone!(state => move || {
                        state.play_asset.set(None);
                    });
                    PlayerPopup::new_default_player_options(
                        jig_id.into(),
                        PreviewPopupCallbacks::new(close)
                    ).render(None)
                })
            }),
        ))
    }
}

fn jump_to(el: HtmlElement, selector: &str) -> Option<()> {
    let el = el
        .get_root_node()
        .dyn_into::<ShadowRoot>()
        .ok()?
        .query_selector(selector)
        .ok()??;
    el.scroll_into_view_with_scroll_into_view_options(
        ScrollIntoViewOptions::new().behavior(ScrollBehavior::Smooth),
    );
    Some(())
}
