use std::rc::Rc;

use components::{
    file_input::{FileInput, FileInputConfig},
    stickers::video::ext::YoutubeUrlExt,
};
use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{not, Signal, SignalExt},
};

use shared::domain::module::body::_groups::design::{VideoHost, YoutubeUrl, YoutubeVideo};
use utils::{component::Component, events};
use web_sys::{HtmlElement, HtmlInputElement};

use crate::edit::pro_dev::unit_editor::add_unit_value::add_video::actions;

use super::state::AddVideo;

const STR_ERROR_MSG_TYPE: &str =
    "Oh no! We don't accept that type of file. We accept all image, audio and PDF files.";
const STR_ERROR_MSG_SIZE: &str = "Oh no! This file is too heavy. Maximum file size: 5 MB.";

impl AddVideo {
    pub fn render(state: &Rc<Self>) -> Dom {
        html!("video-third-party-input-card", {
            .prop("host", "youtube")
            .child(html!("input-wrapper" => HtmlElement, {
                .with_node!(wrapper => {
                    .prop("slot", "input")
                    .prop("label", "Add a YouTube link")
                    .child(html!("input" => HtmlInputElement, {
                        .prop_signal("value", state.video.signal_cloned().map(|video| {
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

                                        state.save(host);

                                        log::info!("TODO: put in error")
                                    },
                                };
                            }))
                        })
                    }))
                })
            }))
            // .children_signal_vec(state.video.signal_ref(clone!(state => move |video| {
            //     match video {
            //         None => vec![],
            //         Some(video) => vec![
            //             html!("input-checkbox", {
            //                 .prop("label", "Clip video")
            //                 .prop("slot", "clip-checkbox")
            //                 .prop_signal("checked", state.sidebar.base.clip.signal())
            //                 // .prop_signal("disabled", map_ref! {
            //                 //     let start_at = video.start_at.signal(),
            //                 //     let end_at = video.end_at.signal() => {
            //                 //         start_at.is_some() || end_at.is_some()
            //                 //     }
            //                 // })
            //                 .event(clone!(state, video => move |e: events::CustomToggle| {
            //                     if !e.value() {
            //                         // clear values if unchecked
            //                         video.start_at.set(None);
            //                         video.end_at.set(None);
            //                     }
            //                     state.sidebar.base.clip.set(e.value());
            //                 }))
            //             }),
            //             html!("button-rect", {
            //                 .prop("slot", "delete")
            //                 .prop("kind", "text")
            //                 .prop("color", "blue")
            //                 .text(STR_DELETE)
            //                 .event(clone!(state => move |_: events::Click| {
            //                     state.sidebar.base.delete_video();
            //                 }))
            //             }),
            //         ],
            //     }
            // })).to_signal_vec())
            // .children_signal_vec(show_start_end_signal(&state).map(clone!(state => move |video| {
            //     match video {
            //         None => vec![],
            //         Some(video) => vec![
            //             html!("input-wrapper", {
            //                 .prop("slot", "start-at")
            //                 .prop("label", "Start time")
            //                 .child(html!("input-minutes-seconds" => HtmlElement, {
            //                     .prop("type", "number")
            //                     .prop_signal("value", video.start_at.signal_ref(|start_at| {
            //                         match start_at {
            //                             Some(start_at) => start_at.to_string(),
            //                             None => String::new(),
            //                         }
            //                     }))
            //                     .event(clone!(state => move |e: events::CustomInputNumber| {
            //                         let video = state.sidebar.base.get_video_sticker().unwrap_ji();

            //                         let value = e.value().map(|num| num as u32);

            //                         log::info!("{value:?}");

            //                         video.start_at.set(value);
            //                         Stickers::call_change(&Rc::clone(&state.sidebar.base.stickers));
            //                     }))
            //                 }))
            //             }),
            //         ],
            //     }
            // })).to_signal_vec())
        })
    }
}
