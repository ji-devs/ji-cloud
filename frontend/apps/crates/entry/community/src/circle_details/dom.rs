use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal_vec::SignalVecExt;
use shared::{
    domain::user::{public_user::PublicUser, UserId},
    media::MediaLibrary,
};
use utils::{
    events,
    routes::{CommunityMembersRoute, CommunityRoute, Route},
};
use wasm_bindgen::JsValue;

use crate::state::MEMBER_LIST_GRID_COLUMNS;

use super::CircleDetails;

// const STR_CONTACT_ADMIN: &str = "Contact admin";
// const STR_INVITE: &str = "Invite";
const STR_MEMBER: &str = "Member";
const STR_JOIN: &str = "Join";
const STR_SEARCH_MEMBER: &str = "Search member";

impl CircleDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_data();

        html!("empty-fragment", {
            .child_signal(state.circle.signal_ref(clone!(state => move |circle| {
                circle.as_ref().map(|circle| {
                    html!("community-circle-details", {
                        .property("name", &circle.display_name)
                        .property("description", &circle.description)
                        .property("memberCount", circle.member_count)
                        .children(&mut [
                            html!("img-ji", {
                                .property("slot", "img")
                                .property("lib", MediaLibrary::User.to_str())
                                .apply(|dom| match circle.image {
                                    Some(image) => dom.property("id", &image.0.to_string()),
                                    None => dom,
                                })
                            }),
                            // html!("button-rect", {
                            //     .property("slot", "actions")
                            //     .property("kind", "outline")
                            //     .property("size", "small")
                            //     .property("color", "blue")
                            //     .text(STR_CONTACT_ADMIN)
                            // }),
                            // html!("button-rect", {
                            //     .property("slot", "actions")
                            //     .property("kind", "outline")
                            //     .property("size", "small")
                            //     .property("color", "blue")
                            //     .text(STR_INVITE)
                            // }),
                            // member-images
                            html!("input-search", {
                                .property("slot", "member-search")
                                .property("placeholder", STR_SEARCH_MEMBER)
                            }),
                        ])
                        .child_signal(state.community_state.user.signal_ref(clone!(state => move |user| {
                            let is_member = match user {
                                Some(user) => user.circles.iter().any(|circle| circle == &state.circle_id),
                                None => false,
                            };
                            Some(match is_member {
                                true => {
                                    html!("button-rect", {
                                        .property("slot", "actions")
                                        .property("kind", "outline")
                                        .property("size", "small")
                                        .property("color", "green")
                                        .child(html!("fa-icon", {
                                            .property("icon", "fa-solid fa-check")
                                        }))
                                        .text(STR_MEMBER)
                                        .event(clone!(state => move |_: events::Click| {
                                            state.leave_circle();
                                        }))
                                    })
                                },
                                false => {
                                    html!("button-rect", {
                                        .property("slot", "actions")
                                        .property("kind", "outline")
                                        .property("size", "small")
                                        .property("color", "blue")
                                        .text(STR_JOIN)
                                        .event(clone!(state => move |_: events::Click| {
                                            state.join_circle();
                                        }))
                                    })
                                },
                            })
                        })))
                        .children_signal_vec(state.members.signal_vec_cloned().map(clone!(state => move |member| {
                            state.render_member(&member)
                        })))
                    })
                })
            })))
        })
    }

    fn render_member(self: &Rc<Self>, member: &PublicUser) -> Dom {
        html!("community-list-member", {
            .property("slot", "members")
            .class(&*MEMBER_LIST_GRID_COLUMNS)
            .property("slot", "items")
            .property("name", &format!("{} {}", member.given_name, member.family_name))
            // .property("city", "New York")
            // .property("state", "NY")
            .apply(|mut dom| {
                if let Some(language) = &member.language {
                    dom = dom.property("language", language);
                };
                dom
            })
            .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(UserId(member.id)))).to_string()
            }))
            .child(html!("profile-image", {
                .property("slot", "img")
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
