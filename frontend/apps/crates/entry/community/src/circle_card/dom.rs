use dominator::{clone, events, html, shadow_root, Dom, EventOptions};
use futures_signals::signal::{Mutable, SignalExt};
use shared::media::MediaLibrary;
use utils::routes::{CommunityCirclesRoute, CommunityRoute, Route};
use web_sys::ShadowRootMode;

use super::{actions, CircleCard};

impl CircleCard<'_> {
    pub fn render(self) -> Dom {
        let circle_id = self.circle.id;
        let joined_mutable = Mutable::new(self.circle.joined);
        let route = Route::Community(CommunityRoute::Circles(CommunityCirclesRoute::Circle(
            circle_id,
        )));

        html!("div", {
            .shadow_root!(ShadowRootMode::Open => {
                .child(html!("style", {
                    .text(&include_str!("./styles.css"))
                }))
                .child(html!("a", {
                    .class("main-link")
                    .attr("href", &route.to_string())
                    .event_with_options(&EventOptions {bubbles: true, preventable: true}, clone!(route => move |e: events::Click| {
                        e.prevent_default();
                        route.go_to();
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
                            .child_signal(joined_mutable.signal().map(move |following| {
                                Some(match following {
                                    true => {
                                        html!("button-rect", {
                                            .prop("kind", "outline")
                                            .prop("color", "green")
                                            .child(html!("fa-icon", {
                                                .prop("icon", "fa-regular fa-check")
                                            }))
                                            .text("Member")
                                            .event_with_options(&EventOptions {bubbles: true, preventable: true}, clone!(joined_mutable, circle_id => move |e: events::Click| {
                                                e.prevent_default();
                                                e.stop_propagation();
                                                joined_mutable.set(false);
                                                actions::leave_circle(circle_id);
                                            }))
                                        })
                                    },
                                    false => {
                                        html!("button-rect", {
                                            .prop("kind", "outline")
                                            .prop("color", "grey")
                                            .text("Join")
                                            .event_with_options(&EventOptions {bubbles: true, preventable: true}, clone!(joined_mutable, circle_id => move |e: events::Click| {
                                                e.prevent_default();
                                                e.stop_propagation();
                                                joined_mutable.set(true);
                                                actions::join_circle(circle_id);
                                            }))
                                        })
                                    },
                                })
                            }))
                        }))
                    }))
                }))
            })
        })
    }
}
