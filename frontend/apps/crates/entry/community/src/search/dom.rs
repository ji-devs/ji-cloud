use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::{map_ref, signal::Signal, signal_vec::SignalVecExt};
use shared::domain::{badge::Badge, user::public_user::PublicUser};
use utils::{
    events,
    routes::{CommunityCirclesRoute, CommunityMembersRoute, CommunityRoute, Route},
};
use wasm_bindgen::JsValue;

use super::CommunitySearch;

use crate::state::{CIRCLE_LIST_GRID_COLUMNS, MEMBER_LIST_GRID_COLUMNS};

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
            .property_signal("circleCount", state.member_count.signal())
            .children_signal_vec(state.circles.signal_vec_cloned().map(clone!(state => move|circle| {
                state.render_circle(&circle)
            })))
            .child_signal(state.render_see_more_circles())
        })
    }

    fn render_member(self: &Rc<Self>, member: &PublicUser) -> Dom {
        html!("community-list-member", {
            .class(&*MEMBER_LIST_GRID_COLUMNS)
            .property("slot", "members")
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

    fn render_circle(self: &Rc<Self>, circle: &Badge) -> Dom {
        html!("community-list-circle", {
            .class(&*CIRCLE_LIST_GRID_COLUMNS)
            .property("slot", "circles")
            .property("name", &circle.display_name)
            .property("member-count", circle.member_count)
            .property("description", &circle.description)
            .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                Route::Community(CommunityRoute::Circles(CommunityCirclesRoute::Circle(circle.id))).to_string()
            }))
            .child(html!("img", {
                .property("slot", "img")
                .property("src", circle.thumbnail.as_str())
            }))
            .child(html!("community-list-circle-status", {
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

    fn render_see_more_circles(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = Rc::clone(self);
        map_ref! {
            let circle_count = state.circle_count.signal(),
            let circle_len = state.circles.signal_vec_cloned().len() => move {
                if *circle_count > *circle_len as u32 {
                    Some(html!("button-rect", {
                        .property("slot", "circles-see-more")
                        .property("color", "blue")
                        .property_signal("disabled", state.loader.is_loading())
                        .text(STR_SEE_MORE)
                        .event(clone!(state => move |_: events::Click| {
                            state.load_more_circles();
                        }))
                    }))
                } else {
                    None
                }
            }
        }
    }
}
