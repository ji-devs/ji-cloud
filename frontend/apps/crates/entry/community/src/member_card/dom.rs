use dominator::{html, Dom, shadow_root};
use itertools::Itertools;
use utils::{location::Country, languages::Language};
use wasm_bindgen::JsValue;
use web_sys::ShadowRootMode;

use super::MemberCard;

impl MemberCard<'_> {
    pub fn render(self) -> Dom {
        html!("div", {
            .prop("slot", self.slot)
            .shadow_root!(ShadowRootMode::Open => {
                .child(html!("style", {
                    .text(&include_str!("./styles.css"))
                }))
                .apply(|mut dom| {
                    if let Some(menu) = self.menu {
                        dom = dom.child(html!("div", {
                            .class("menu")
                            .child(menu)
                        }));
                    }
                    dom
                })
                .child(html!("div", {
                    .class("admin-indicator")
                    .child(html!("img-ui", {
                        .prop("path", "entry/community/circle-admin-icon.svg")
                    }))
                    .text("Admin")
                }))
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
                                        if let Some(country) = Country::from_google_location(&self.member.location) {
                                            dom = dom.prop("title", country.name);
                                            dom = dom.child(html!("span", {
                                                .text(&country.code)
                                            }));
                                            dom = dom.child(html!("img-ui", {
                                                .prop("path", format!("flags/{}.webp", country.code))
                                            }));
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
                                    .text("296")
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
                        html!("button-rect", {
                            .prop("kind", "outline")
                            .prop("color", "green")
                            .child(html!("fa-icon", {
                                .prop("icon", "fa-regular fa-check")
                            }))
                            .text("Following")
                        }),
                    ])
                }))
            })
        })
    }
}
