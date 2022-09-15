use std::rc::Rc;

use components::stickers::{state::Stickers, video::ext::YoutubeUrlExt};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::_groups::design::{VideoHost, YoutubeUrl, YoutubeVideo};
use utils::{events, unwrap::UnwrapJiExt};
use web_sys::{HtmlElement, HtmlInputElement};

use crate::base::sidebar::step_2::actions;

use super::super::state::Step2;

const STR_DELETE: &str = "Delete";

pub fn render(state: Rc<Step2>) -> Dom {
    html!("video-third-party-input-card", {
        .property("host", "youtube")
        .child(html!("input-wrapper" => HtmlElement, {
            .with_node!(wrapper => {
                .property("slot", "input")
                .property("label", "Add a YouTube link")
                .child(html!("input" => HtmlInputElement, {
                    .property_signal("value", state.sidebar.base.video.signal_cloned().map(|video| {
                        match video {
                            None => String::new(),
                            Some(video) => {
                                // TODO: don't .lock_ref(), use ref_map
                                match &*video.host.lock_ref() {
                                    VideoHost::Youtube(youtube) => youtube.url.0.clone(),
                                }
                            },
                        }
                    }))
                    .with_node!(input => {
                        .event(clone!(state => move |_: events::Input| {
                            match YoutubeUrl::try_parse(input.value()) {
                                Err(_) => {
                                    actions::set_error(&wrapper, true);
                                }
                                Ok(youtube_url) => {
                                    actions::set_error(&wrapper, false);
                                    let host = VideoHost::Youtube(YoutubeVideo {
                                        url: youtube_url,
                                    });
                                    state.sidebar.base.on_link_change(host);
                                },
                            };
                        }))
                    })
                }))
            })
        }))
        .children_signal_vec(state.sidebar.base.video.signal_ref(clone!(state => move |video| {
            match video {
                None => vec![],
                Some(video) => vec![
                    html!("input-wrapper", {
                        .property("slot", "start-at")
                        .property("label", "Start time")
                        .child(html!("input-minutes-seconds" => HtmlElement, {
                            .property("type", "number")
                            .property_signal("value", video.start_at.signal_ref(|start_at| {
                                match start_at {
                                    Some(start_at) => start_at.to_string(),
                                    None => String::new(),
                                }
                            }))
                            .with_node!(_input => {
                                .event(clone!(state => move |e: events::CustomInputNumber| {
                                    let video = state.sidebar.base.get_video_sticker().unwrap_ji();

                                    let value = e.value()
                                        .map(|num| num as u32);

                                        log::info!("{value:?}");

                                    video.start_at.set(value);
                                    Stickers::call_change(&Rc::clone(&state.sidebar.base.stickers));
                                }))
                            })
                        }))
                    }),
                    html!("input-wrapper", {
                        .property("slot", "end-at")
                        .property("label", "End time")
                        .child(html!("input-minutes-seconds" => HtmlElement, {
                            .property("type", "number")
                            .property_signal("value", video.end_at.signal_ref(|end_at| {
                                match end_at {
                                    Some(end_at) => end_at.to_string(),
                                    None => String::new(),
                                }
                            }))
                            .with_node!(_input => {
                                .event(clone!(state => move |e: events::CustomInputNumber| {
                                    let video = state.sidebar.base.get_video_sticker().unwrap_ji();

                                    let value = e.value()
                                        .map(|num| num as u32);

                                        log::info!("{value:?}");

                                    video.end_at.set(value);
                                    Stickers::call_change(&Rc::clone(&state.sidebar.base.stickers));
                                }))
                            })
                        }))
                    }),
                    html!("button-rect", {
                        .property("slot", "delete")
                        .property("kind", "text")
                        .property("color", "blue")
                        .text(STR_DELETE)
                        .event(clone!(state => move |_: events::Click| {
                            state.sidebar.base.delete_video();
                        }))
                    }),
                ],
            }
        })).to_signal_vec())
    })
}
