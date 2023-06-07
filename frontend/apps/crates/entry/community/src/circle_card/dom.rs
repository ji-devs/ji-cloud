use dominator::{html, Dom, shadow_root};
use futures_signals::signal::SignalExt;
use shared::media::MediaLibrary;
use web_sys::ShadowRootMode;

use super::CircleCard;

impl CircleCard<'_> {
    pub fn render(self) -> Dom {
        html!("div", {
            .prop("slot", self.slot)
            .shadow_root!(ShadowRootMode::Open => {
                .child(html!("style", {
                    .text(&include_str!("./styles.css"))
                }))
                .child(html!("div", {
                    .class("logo")
                    .child(html!("img-ji", {
                        .prop("lib", MediaLibrary::User.to_str())
                        .prop("id", &self.circle.image.0.to_string())
                    }))
                }))
                .child(html!("div", {
                    .class("color-indicator")
                    .class("member")
                }))
                .child(html!("div", {
                    .class("main")
                    .child(html!("p", {
                        .class("name")
                        .text(&self.circle.display_name)
                    }))
                    .child(html!("hr"))
                    .child(html!("div", {
                        .class("bottom-line")
                        .child(html!("div", {
                            .class("member-count")
                            .child(html!("img-ui", {
                                .prop("path", "entry/community/circle-icon.svg")
                            }))
                            .text(&self.circle.member_count.to_string())
                        }))
                        .child_signal(self.is_member.signal().map(|is_member| {
                            Some(match is_member {
                                true => {
                                    html!("button-rect", {
                                        .prop("color", "green")
                                        .prop("kind", "outline")
                                        .child(html!("fa-icon", {
                                            .prop("icon", "fa-regular fa-check")
                                        }))
                                        .text("Member")
                                    })
                                },
                                false => {
                                    html!("button-rect", {
                                        .prop("color", "grey")
                                        .prop("kind", "outline")
                                        .text("Join")
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
