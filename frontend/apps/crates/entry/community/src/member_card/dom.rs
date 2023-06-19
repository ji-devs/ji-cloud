use dominator::{clone, events, html, shadow_root, Dom, EventOptions};
use futures_signals::signal::{Mutable, SignalExt};
use itertools::Itertools;
use utils::{
    languages::Language,
    routes::{CommunityMembersRoute, CommunityRoute, Route},
};
use wasm_bindgen::JsValue;
use web_sys::ShadowRootMode;

use super::{actions, MemberCard};

impl MemberCard<'_> {
    pub fn render(self) -> Dom {
        let member_id = self.member.id;
        let following_mutable = Mutable::new(false);
        let path = Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(
            member_id,
        )))
        .to_string();

        html!("div", {
            .prop("slot", self.slot)
            .shadow_root!(ShadowRootMode::Open => {
                .child(html!("style", {
                    .text(&include_str!("./styles.css"))
                }))
                .child(html!("a", {
                    .class("main-link")
                    .attr("href", &path)
                    .event_with_options(&EventOptions {bubbles: true, preventable: true}, move |e: events::Click| {
                        e.prevent_default();
                        dominator::routing::go_to_url(&path);
                    })
                    .apply(|mut dom| {
                        if let Some(menu) = self.menu {
                            dom = dom.child(html!("div", {
                                .event_with_options(&EventOptions{ bubbles: true, preventable: true }, move |e: events::Click| {
                                    e.prevent_default();
                                    e.stop_propagation();
                                })
                                .class("menu")
                                .child(menu)
                            }));
                        }
                        dom
                    })
                    .apply_if(self.admin_tag, |dom| {
                        dom.child(html!("div", {
                            .class("admin-indicator")
                            .child(html!("img-ui", {
                                .prop("path", "entry/community/circle-admin-icon.svg")
                            }))
                            .text("Admin")
                        }))
                    })
                    .child(html!("div", {
                        .class("main")
                        .children(&mut [
                            html!("div", {
                                .class("top-section")
                                .children(&mut [
                                    html!("profile-image", {
                                        .prop("imageId", {
                                            match &self.member.profile_image {
                                                Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                                                None => JsValue::UNDEFINED,
                                            }
                                        })
                                        .prop("givenName", &self.member.given_name)
                                        .prop("familyName", &self.member.family_name)
                                        .apply(|mut dom| {
                                            if let Some(badge) = &self.member.badge {
                                                dom = dom.prop("badge", badge.as_str());
                                            }
                                            dom
                                        })
                                    }),
                                    html!("div", {
                                        .class("name")
                                        .text(&format!("{} {}", self.member.given_name, self.member.family_name))
                                        // .child(html!("div", {
                                        //     .class("given-name")
                                        //     .text(&self.member.given_name)
                                        // }))
                                        // .child(html!("div", {
                                        //     .class("family-name")
                                        //     .text(&self.member.family_name)
                                        // }))
                                    }),
                                    html!("div", {
                                        .class("country")
                                        .apply(|mut dom| {
                                            if let Some(country_short) = &self.member.country_short {
                                                dom = dom.child(html!("span", {
                                                    .text(&country_short)
                                                }));
                                                dom = dom.child(html!("img-ui", {
                                                    .prop("path", format!("flags/{}.webp", country_short))
                                                }));
                                            }
                                            if let Some(country_long) = &self.member.country_long {
                                                dom = dom.prop("title", country_long);
                                            }
                                            dom
                                        })
                                    })
                                ])
                            }),
                            html!("hr"),
                            html!("div", {
                                .class("key-values")
                                .children(&mut [
                                    html!("span", {
                                        .class("key")
                                        .text("Creations")
                                    }),
                                    html!("span", {
                                        .class("value")
                                        .text(&match self.member.total_asset_count {
                                            0 => String::new(),
                                            count => count.to_string()
                                        })
                                    }),
                                    html!("span", {
                                        .class("key")
                                        .text("Languages")
                                    }),
                                    html!("span", {
                                        .class("value")
                                        .apply(|mut dom| {
                                            if let Some(languages_spoken) = &self.member.languages_spoken {
                                                let languages = languages_spoken.iter().map(|l| Language::code_to_display_name(l)).join(", ");
                                                dom = dom.child(html!("div", {
                                                    .text(&languages)
                                                }));
                                            }
                                            dom
                                        })
                                    }),
                                ])
                            }),
                        ])
                        .child_signal(following_mutable.signal().map(move |following| {
                            Some(match following {
                                true => {
                                    html!("button-rect", {
                                        .prop("kind", "outline")
                                        .prop("color", "green")
                                        .child(html!("fa-icon", {
                                            .prop("icon", "fa-regular fa-check")
                                        }))
                                        .text("Following")
                                        .event_with_options(&EventOptions {bubbles: true, preventable: true}, clone!(following_mutable, member_id => move |e: events::Click| {
                                            e.prevent_default();
                                            e.stop_propagation();
                                            following_mutable.set(false);
                                            actions::unfollow_user(member_id);
                                        }))
                                    })
                                },
                                false => {
                                    html!("button-rect", {
                                        .prop("kind", "outline")
                                        .prop("color", "grey")
                                        .text("Follow")
                                        .event_with_options(&EventOptions {bubbles: true, preventable: true}, clone!(following_mutable, member_id => move |e: events::Click| {
                                            e.prevent_default();
                                            e.stop_propagation();
                                            following_mutable.set(true);
                                            actions::follow_user(member_id);
                                        }))
                                    })
                                },
                            })
                        }))
                    }))
                }))
            })
        })
    }
}
