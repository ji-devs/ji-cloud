use components::stickers::{
    state::Stickers,
    video::{ext::YoutubeUrlExt, state::Video},
};
use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::module::body::_groups::design::{VideoHost, YoutubeUrl, YoutubeVideo};
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};
use web_sys::{HtmlElement, HtmlInputElement};

use crate::base::sidebar::step_2::actions;

use super::super::state::Step2;

const STR_DELETE: &str = "Delete";

pub fn render(state: Rc<Step2>) -> Dom {
    html!("video-third-party-input-card", {
        .prop("host", "youtube")
        .child(html!("input-wrapper" => HtmlElement, {
            .with_node!(wrapper => {
                .prop("slot", "input")
                .prop("label", "Add a YouTube link")
                .child(html!("input" => HtmlInputElement, {
                    .prop_signal("value", state.sidebar.base.video.signal_cloned().map(|video| {
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
                    html!("input-checkbox", {
                        .prop("label", "Clip video")
                        .prop("slot", "clip-checkbox")
                        .prop_signal("checked", state.sidebar.base.clip.signal())
                        // .prop_signal("disabled", map_ref! {
                        //     let start_at = video.start_at.signal(),
                        //     let end_at = video.end_at.signal() => {
                        //         start_at.is_some() || end_at.is_some()
                        //     }
                        // })
                        .event(clone!(state, video => move |e: events::CustomToggle| {
                            if !e.value() {
                                // clear values if unchecked
                                video.start_at.set(None);
                                video.end_at.set(None);
                            }
                            state.sidebar.base.clip.set(e.value());
                        }))
                    }),
                    html!("button-rect", {
                        .prop("slot", "delete")
                        .prop("kind", "text")
                        .prop("color", "blue")
                        .text(STR_DELETE)
                        .event(clone!(state => move |_: events::Click| {
                            state.sidebar.base.delete_video();
                        }))
                    }),
                ],
            }
        })).to_signal_vec())
        .children_signal_vec(show_start_end_signal(&state).map(clone!(state => move |video| {
            match video {
                None => vec![],
                Some(video) => vec![
                    html!("input-wrapper", {
                        .prop("slot", "start-at")
                        .prop("label", "Start time")
                        .child(html!("input-minutes-seconds" => HtmlElement, {
                            .prop("type", "number")
                            .prop_signal("value", video.start_at.signal_ref(|start_at| {
                                match start_at {
                                    Some(start_at) => start_at.to_string(),
                                    None => String::new(),
                                }
                            }))
                            .event(clone!(state => move |e: events::CustomInputNumber| {
                                let video = state.sidebar.base.get_video_sticker().unwrap_ji();

                                let value = e.value().map(|num| num as u32);

                                log::info!("{value:?}");

                                video.start_at.set(value);
                                Stickers::call_change(&Rc::clone(&state.sidebar.base.stickers));
                            }))
                        }))
                    }),
                    html!("input-wrapper", {
                        .prop("slot", "end-at")
                        .prop("label", "End time")
                        .child(html!("input-minutes-seconds" => HtmlElement, {
                            .prop("type", "number")
                            .prop_signal("value", video.end_at.signal_ref(|end_at| {
                                match end_at {
                                    Some(end_at) => end_at.to_string(),
                                    None => String::new(),
                                }
                            }))
                            .event(clone!(state => move |e: events::CustomInputNumber| {
                                let video = state.sidebar.base.get_video_sticker().unwrap_ji();

                                let value = e.value().map(|num| num as u32);

                                log::info!("{value:?}");

                                video.end_at.set(value);
                                Stickers::call_change(&Rc::clone(&state.sidebar.base.stickers));
                            }))
                        }))
                    }),
                ],
            }
        })).to_signal_vec())
    })
}

fn show_start_end_signal(state: &Rc<Step2>) -> impl Signal<Item = Option<Rc<Video>>> {
    map_ref! {
        let video = state.sidebar.base.video.signal_cloned(),
        let clip = state.sidebar.base.clip.signal() => move {
            match video {
                Some(video) if *clip => Some(Rc::clone(video)),
                _ => None,
            }
        }
    }
}
