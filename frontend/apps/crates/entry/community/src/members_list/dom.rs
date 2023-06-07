use std::rc::Rc;

use dominator::{class, clone, html, pseudo, with_node, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use itertools::Itertools;
use shared::domain::user::public_user::PublicUser;
use utils::{
    events,
    languages::Language,
    location::Country,
    routes::{CommunityMembersRoute, CommunityRoute, Route},
};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;

use super::MembersList;

impl MembersList {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_members();

        html!("community-list", {
            .prop("header", "Connect with members")
            .child(html!("community-list-member-header", {
                .prop("slot", "sort-header")
            }))
            .child(html!("community-pagination", {
                .prop("slot", "sort-header")
                .prop_signal("total", state.total_pages.signal())
                .children(&mut [
                    html!("fa-button", {
                        .prop("slot", "back")
                        .prop("icon", "fa-solid fa-angle-left")
                        .prop_signal("disabled", state.active_page.signal().map(|active_page| {
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
                            .prop("slot", "active-page")
                            .prop("type", "number")
                            .prop("min", 1)
                            .prop_signal("max", state.total_pages.signal())
                            .prop_signal("value", state.active_page.signal().map(|active_page| {
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
                        .prop("slot", "forward")
                        .prop("icon", "fa-solid fa-angle-right")
                        .prop_signal("disabled", map_ref! {
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
                            .prop("slot", "items")
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
            .prop("slot", "items")
            .prop("name", &format!("{} {}", member.given_name, member.family_name))
            .apply(|mut dom| {
                if let Some(country) = Country::from_google_location(&member.location) {
                    dom = dom.prop("countryCode", country.code);
                    dom = dom.prop("countryName", country.name);
                }
                dom
            })
            .apply(|mut dom| {
                if let Some(languages_spoken) = &member.languages_spoken {
                    if languages_spoken.len() > 0 {
                        let languages = languages_spoken.iter().map(|l| Language::code_to_display_name(l)).join(", ");
                        dom = dom.prop("language", languages);
                    }
                };
                dom
            })
            .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(member.id))).to_string()
            }))
            .child(html!("profile-image", {
                .prop("slot", "img")
                .prop("imageId", {
                    match &member.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
                .prop("givenName", &member.given_name)
                .prop("familyName", &member.family_name)
            }))
        })
        // let state = self;
        // MemberCard {
        //     member,
        //     slot: "items",
        //     following: Mutable::new(false).read_only(),
        //     on_follow: Box::new(clone!(state => move|following| {

        //     })),
        //     menu: None,
        // }.render()
    }
}
