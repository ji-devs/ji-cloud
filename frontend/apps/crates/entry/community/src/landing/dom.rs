use std::rc::Rc;

use components::{
    asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::signal::SignalExt;
use shared::domain::user::public_user::PublicUser;
use utils::{
    component::Component,
    routes::{CommunityCirclesRoute, CommunityMembersRoute, CommunityRoute, Route},
};
use wasm_bindgen::JsValue;
use web_sys::ShadowRoot;

use crate::circle_card::CircleCard;

use super::CommunityLanding;

const WELCOME_VIDEO_ID: &str = "vslI-xddVh8";
const STR_SEE_ALL_MEMBERS: &str = "See all members";
const STR_SEE_ALL_CIRCLES: &str = "See all circles";
const STR_SEE_ALL_COURSES: &str = "See all courses";

impl Component<CommunityLanding> for Rc<CommunityLanding> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        state.load_data();

        dom
            .child(html!("div", {
                .class("main")
                .child(html!("section", {
                    .class("top-members")
                    .child(html!("div", {
                        .class("header")
                        .child(html!("h3", {
                            .child(html!("img-ui", {
                                .prop("path", "entry/community/landing/member.svg")
                            }))
                            .text("Jigzi’s top 10 creators")
                        }))
                    }))
                    .child(html!("div", {
                        .class("members")
                        .child(html!("div", {
                            .class("row")
                            .child(html!("span", {
                                .class("cell-header")
                                .class("index")
                            }))
                            .child(html!("span", {
                                .class("cell-header")
                                .class("image")
                            }))
                            .child(html!("span", {
                                .class("cell-header")
                                .class("name")
                                .text("Name")
                            }))
                            .child(html!("span", {
                                .class("cell-header")
                                .class("flag")
                                .text("Country")
                            }))
                            .child(html!("span", {
                                .class("cell-header")
                                .class("creation-count")
                                .text("Creations")
                            }))
                        }))
                        .children_signal_vec(state.top_members.signal_ref(clone!(state => move |top_members| {
                            match top_members {
                                None => vec![html!("progress")],
                                Some(top_members) => {
                                    top_members.iter().enumerate().map(|(index, member)| {
                                        state.render_member(member, index)
                                    }).collect()
                                },
                            }
                        })).to_signal_vec())
                    }))
                    .child(html!("button-rect", {
                        .prop("color", "blue")
                        .text(STR_SEE_ALL_MEMBERS)
                        .apply(move |dom| utils::on_click_go_to_url!(dom, {
                            Route::Community(CommunityRoute::Members(CommunityMembersRoute::List))
                        }))
                    }))
                }))
                .child(html!("video-youtube-player", {
                    .prop("videoId", WELCOME_VIDEO_ID)
                    .prop("autoplay", true)
                    .prop("muted", true)
                    .prop("loop", true)
                }))
                .child(html!("section", {
                    .class("top-circles")
                    .child(html!("div", {
                        .class("header")
                        .child(html!("h3", {
                            .child(html!("img-ui", {
                                .prop("path", "entry/community/landing/circle.svg")
                            }))
                            .text("Popular circles")
                        }))
                    }))
                    .child(html!("div", {
                        .class("circles")
                        .children_signal_vec(state.top_circles.signal_ref(move |top_circles| {
                            match top_circles {
                                None => vec![html!("progress")],
                                Some(top_circles) => {
                                    top_circles.iter().map(|circle| {
                                        CircleCard {
                                            circle,
                                        }.render()
                                    }).collect()
                                },
                            }
                        }).to_signal_vec())
                    }))
                    .child(html!("button-rect", {
                        .prop("color", "blue")
                        .text(STR_SEE_ALL_CIRCLES)
                        .apply(move |dom| utils::on_click_go_to_url!(dom, {
                            Route::Community(CommunityRoute::Circles(CommunityCirclesRoute::List))
                        }))
                    }))
                }))
            }))
            .child(html!("section", {
                .class("top-courses")
                .child(html!("div", {
                    .class("header")
                    .child(html!("h3", {
                        .child(html!("img-ui", {
                            .prop("path", "entry/community/landing/course.svg")
                        }))
                        .text("Courses")
                    }))
                    .child(html!("button-rect", {
                        .prop("color", "blue")
                        .prop("kind", "text")
                        .text(STR_SEE_ALL_COURSES)
                        .apply(move |dom| utils::on_click_go_to_url!(dom, {
                            Route::Community(CommunityRoute::Courses)
                        }))
                    }))
                }))
                .child(html!("div", {
                    .class("courses")
                    .children_signal_vec(state.top_courses.signal_ref(clone!(state => move |top_courses| {
                        match top_courses {
                            None => vec![html!("progress")],
                            Some(top_courses) => {
                                top_courses.into_iter().map(clone!(state => move |courses| {
                                    let courses_id = courses.id;
                                    html!("div", {
                                        .style("cursor", "pointer")
                                        .child(render_asset_card(
                                            &courses.clone().into(),
                                            AssetCardConfig {
                                                bottom_indicator: AssetCardBottomIndicator::Author,
                                                dense: true,
                                                ..Default::default()
                                            }
                                        ))
                                        .event(clone!(state => move |_: events::Click| {
                                            state.play_course.set(Some(courses_id));
                                        }))
                                    })
                                })).collect()
                            },
                        }
                    })).to_signal_vec())
                }))
            }))
            .child_signal(state.play_course.signal().map(clone!(state => move |play_course| {
                play_course.map(|course_id| {
                    PlayerPopup::new_default_player_options(
                        course_id.into(),
                        PreviewPopupCallbacks {
                            close: Box::new(clone!(state => move|| {
                                state.play_course.set(None);
                            }))
                        },
                    ).render(None)
                })
            })))
    }
}

impl CommunityLanding {
    fn render_member(self: &Rc<Self>, member: &PublicUser, index: usize) -> Dom {
        utils::link!(Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(member.id))), {
            .class("row")
            .child(html!("span", {
                .class("cell")
                .class("index")
                .text(&(index + 1).to_string())
            }))
            .child(html!("span", {
                .class("cell")
                .class("image")
                .child(html!("profile-image", {
                    .prop("imageId", {
                        match &member.profile_image {
                            Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                            None => JsValue::UNDEFINED,
                        }
                    })
                    .prop("givenName", &member.given_name)
                    .prop("familyName", &member.family_name)
                }))
            }))
            .child(html!("span", {
                .class("cell")
                .class("name")
                .text(&format!("{} {}", member.given_name, member.family_name))
            }))
            .child(html!("span", {
                .class("cell")
                .class("flag")
                .apply(|mut dom| {
                    if let Some(country_short) = &member.country_short {
                        dom = dom.child(html!("img-ui", {
                            .prop("path", format!("flags/{}.webp", country_short))
                        }))
                    }
                    dom
                })
            }))
            .child(html!("span", {
                .class("cell")
                .class("creation-count")
                .text(&member.total_asset_count.to_string())
            }))
        })
    }
}
