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
            .children_signal_vec(state.members.signal_vec_cloned().map(clone!(state => move |member| {
                state.render_member(member)
            })))
        })
    }

    fn render_member(self: &Rc<Self>, member: PublicUser) -> Dom {
        fn span(s: &str) -> Dom {
            html!("span", {
                .text(s)
            })
        }

        html!("div", {
            .style("display", "flex")
            .style("column-gap", "10px")
            .style("align-items", "center")
            .property("slot", "items")
            .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(member.id))).to_string()
            }))
            .child(html!("profile-image", {
                .style("height", "40px")
                .style("width", "40px")
                .property("imageId", {
                    match &member.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
            }))
            .child(span(&member.given_name))
            .child(span(&member.family_name))
            .child(span(&member.language.unwrap_or_default()))
        })
    }
}
