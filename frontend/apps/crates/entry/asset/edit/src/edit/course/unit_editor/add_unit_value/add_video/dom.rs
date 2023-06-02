use std::rc::Rc;

use components::stickers::embed::types::ParseUrlExt;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;

use shared::domain::module::body::_groups::design::{YoutubeEmbed, YoutubeUrl};
use utils::events;
use web_sys::{HtmlElement, HtmlInputElement};

use crate::edit::course::unit_editor::add_unit_value::add_video::actions;

use super::state::AddVideo;

const _STR_ERROR_MSG_TYPE: &str =
    "Oh no! We don't accept that type of file. We accept all image, audio and PDF files.";
const _STR_ERROR_MSG_SIZE: &str = "Oh no! This file is too heavy. Maximum file size: 5 MB.";

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
                                Some(youtube) => {
                                    youtube.url.0.clone()
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
                                        let host = YoutubeEmbed::new(youtube_url);
                                        state.save(host);
                                    },
                                };
                            }))
                        })
                    }))
                })
            }))
        })
    }
}
