use std::rc::Rc;

use dominator::{class, clone, html, pseudo, with_node, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use shared::domain::user::public_user::PublicUser;
use utils::{
    events,
    routes::{CommunityMembersRoute, CommunityRoute, Route},
};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;

use crate::state::MEMBER_LIST_GRID_COLUMNS;

use super::MembersList;

impl MembersList {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_members();

        html!("community-list", {
            .property("header", "Connect with members")
            .child(html!("community-list-member-header", {
                .class(&*MEMBER_LIST_GRID_COLUMNS)
                .property("slot", "sort-header")
            }))
            .child(html!("community-pagination", {
                .property("slot", "sort-header")
                .property_signal("total", state.total_pages.signal())
                .children(&mut [
                    html!("fa-button", {
                        .property("slot", "back")
                        .property("icon", "fa-solid fa-angle-left")
                        .property_signal("disabled", state.active_page.signal().map(|active_page| {
                            active_page <= 1
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            let active_page = state.active_page.get();
                            if active_page > 1 {
                                state.active_page.set(active_page - 1);
                                state.load_members();
                            };
                        }))
                    }),
                    html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .class(class! {
                                .pseudo!("::-webkit-outer-spin-button", {
                                    .style("-webkit-appearance", "none")
                                    .style("margin", "0")
                                })
                                .pseudo!("::-webkit-inner-spin-button", {
                                    .style("-webkit-appearance", "none")
                                    .style("margin", "0")
                                })
                            })
                            .property("slot", "active-page")
                            .property("type", "number")
                            .property("min", 1)
                            .property_signal("value", state.active_page.signal().map(|active_page| {
                                active_page.to_string()
                            }))
                            .event(clone!(state, elem => move |_: events::Input| {
                                let value = elem.value();
                                if let Ok(num) = value.parse::<u32>() {
                                    if num <= state.total_pages.get() {
                                        state.active_page.set(num);
                                        state.load_members();
                                    }
                                };
                            }))
                        })
                    }),
                    html!("fa-button", {
                        .property("slot", "forward")
                        .property("icon", "fa-solid fa-angle-right")
                        .property_signal("disabled", map_ref! {
                            let active_page = state.active_page.signal(),
                            let total_pages = state.total_pages.signal() => {
                                active_page >= total_pages
                            }
                        })
                        .event(clone!(state => move |_: events::Click| {
                            state.active_page.replace_with(|active_page| {
                                *active_page + 1
                            });
                            state.load_members();
                        }))
                    }),
                ])
            }))
            .children_signal_vec(state.members.signal_ref(clone!(state => move|members| {
                match members {
                    None => {
                        vec![html!("progress", {
                            .property("slot", "items")
                        })]
                    },
                    Some(members) => {
                        members.iter().map(|member| {
                            state.render_member(member)
                        }).collect()
                    },
                }
            })).to_signal_vec())
        })
    }

    fn render_member(self: &Rc<Self>, member: &PublicUser) -> Dom {
        html!("community-list-member", {
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
