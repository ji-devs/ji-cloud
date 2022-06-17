use std::rc::Rc;

use dominator::{clone, html, link, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::{badge::Badge, user::public_user::PublicUser};
use utils::routes::{CommunityBadgesRoute, CommunityMembersRoute, CommunityRoute, Route};
use wasm_bindgen::JsValue;

use super::CommunityLanding;

const STR_SEE_MORE: &str = "See more";

impl CommunityLanding {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_data();

        html!("community-landing", {
            .children_signal_vec(state.top_members.signal_ref(clone!(state => move |top_members| {
                match top_members {
                    None => vec![html!("progress", {
                        .property("slot", "members")
                    })],
                    Some(top_members) => {
                        top_members.iter().map(|member| {
                            state.render_member(member)
                        }).collect()
                    },
                }
            })).to_signal_vec())
            .child(html!("button-rect", {
                .property("slot", "members-link")
                .property("color", "blue")
                .text(STR_SEE_MORE)
                .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                    Route::Community(CommunityRoute::Members(CommunityMembersRoute::List)).to_string()
                }))
            }))
            .children_signal_vec(state.top_badges.signal_ref(clone!(state => move |top_badges| {
                match top_badges {
                    None => vec![html!("progress", {
                        .property("slot", "badges")
                    })],
                    Some(top_badges) => {
                        top_badges.iter().map(|badge| {
                            state.render_badge(badge)
                        }).collect()
                    },
                }
            })).to_signal_vec())
            .child(html!("button-rect", {
                .property("slot", "badges-link")
                .property("color", "blue")
                .text(STR_SEE_MORE)
                .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                    Route::Community(CommunityRoute::Badges(CommunityBadgesRoute::List)).to_string()
                }))
            }))
        })
    }

    fn render_member(self: &Rc<Self>, member: &PublicUser) -> Dom {
        link!(Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(member.id))).to_string(), {
            .property("slot", "members")
            .child(html!("profile-image", {
                .style("height", "64px")
                .style("width", "64px")
                .style("overflow", "hidden")
                .style("border-radius", "50%")
                .property("slot", "profile-image")
                .property("imageId", {
                    match &member.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
            }))
            .text(&member.given_name)
        })
    }

    fn render_badge(self: &Rc<Self>, badge: &Badge) -> Dom {
        html!("div", {
            .property("slot", "badges")
            .child(html!("img", {
                .style("height", "90px")
                .style("width", "90px")
                .style("box-shadow", "0 0 8px 0 rgba(0, 0, 0, 0.06)")
                .style("border", "solid 1px var(--light-gray-1)")
                .style("border-radius", "50%")
                .property("src", badge.thumbnail.as_str())
            }))
            .text(&badge.display_name)
        })
    }
}