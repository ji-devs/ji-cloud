use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::{map_ref, signal::Signal, signal_vec::SignalVecExt};
use shared::domain::{badge::Badge, user::public_user::PublicUser};
use utils::{
    events,
    routes::{CommunityBadgesRoute, CommunityMembersRoute, CommunityRoute, Route},
};
use wasm_bindgen::JsValue;

use super::CommunitySearch;

const STR_SEE_MORE: &str = "See more";

impl CommunitySearch {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.search();

        html!("community-search", {
            .property("query", &state.query.q)
            .property_signal("memberCount", state.member_count.signal())
            .children_signal_vec(state.members.signal_vec_cloned().map(clone!(state => move|member| {
                state.render_member(&member)
            })))
            .child_signal(state.render_see_more_members())
            .property_signal("badgeCount", state.member_count.signal())
            .children_signal_vec(state.badges.signal_vec_cloned().map(clone!(state => move|badge| {
                state.render_badge(&badge)
            })))
            .child_signal(state.render_see_more_badges())
        })
    }

    fn render_member(self: &Rc<Self>, member: &PublicUser) -> Dom {
        html!("community-list-member", {
            .property("slot", "members")
            .property("name", &format!("{} {}", member.given_name, member.family_name))
            .property("city", "New York")
            .property("state", "NY")
            .apply(|mut dom| {
                if let Some(language) = &member.language {
                    dom = dom.property("language", language);
                };
                dom
            })
            .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(member.id))).to_string()
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

    fn render_badge(self: &Rc<Self>, badge: &Badge) -> Dom {
        html!("community-list-badge", {
            .property("slot", "badges")
            .property("name", &badge.display_name)
            .property("member-count", badge.member_count)
            .property("description", &badge.description)
            .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                Route::Community(CommunityRoute::Badges(CommunityBadgesRoute::Badge(badge.id))).to_string()
            }))
            .child(html!("img", {
                .property("slot", "img")
                .property("src", badge.thumbnail.as_str())
            }))
            .child(html!("community-list-badge-status", {
                .property("slot", "status")
                .property("status", "")
            }))
        })
    }

    fn render_see_more_members(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = Rc::clone(self);
        map_ref! {
            let member_count = state.member_count.signal(),
            let member_len = state.members.signal_vec_cloned().len() => move {
                if *member_count > *member_len as u32 {
                    Some(html!("button-rect", {
                        .property("slot", "members-see-more")
                        .property("color", "blue")
                        .property_signal("disabled", state.loader.is_loading())
                        .text(STR_SEE_MORE)
                        .event(clone!(state => move |_: events::Click| {
                            state.load_more_members();
                        }))
                    }))
                } else {
                    None
                }
            }
        }
    }

    fn render_see_more_badges(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = Rc::clone(self);
        map_ref! {
            let badge_count = state.badge_count.signal(),
            let badge_len = state.badges.signal_vec_cloned().len() => move {
                if *badge_count > *badge_len as u32 {
                    Some(html!("button-rect", {
                        .property("slot", "badges-see-more")
                        .property("color", "blue")
                        .property_signal("disabled", state.loader.is_loading())
                        .text(STR_SEE_MORE)
                        .event(clone!(state => move |_: events::Click| {
                            state.load_more_badges();
                        }))
                    }))
                } else {
                    None
                }
            }
        }
    }
}
