use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::user::public_user::PublicUser;
use utils::routes::{CommunityMembersRoute, CommunityRoute, Route};
use wasm_bindgen::JsValue;

use super::MembersList;

impl MembersList {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_members();

        html!("community-list", {
            .property("header", "Members")
            .child(html!("community-list-member-header", {
                .property("slot", "sort-header")
            }))
            .children_signal_vec(state.members.signal_vec_cloned().map(clone!(state => move |member| {
                state.render_member(member)
            })))
        })
    }

    fn render_member(self: &Rc<Self>, member: PublicUser) -> Dom {
        html!("community-list-member", {
            .property("slot", "items")
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
}
