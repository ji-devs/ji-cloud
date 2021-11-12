use std::rc::Rc;

use components::stickers::video::ext::YoutubeUrlExt;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::_groups::design::{VideoHost, YoutubeUrl};
use utils::events;
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
                                // TODO: don't .lock_ref()
                                match &*video.host.lock_ref() {
                                    VideoHost::Youtube(youtube_url) => youtube_url.0.clone(),
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
                                    let host = VideoHost::Youtube(youtube_url);
                                    state.sidebar.base.on_link_change(host);
                                },
                            };
                        }))
                    })
                }))
            })
        }))
        .child(html!("button-rect", {
            .property("slot", "delete")
            .property("kind", "text")
            .property("color", "blue")
            .visible_signal(state.sidebar.base.video.signal_cloned().map(|video| video.is_some()))
            .text(STR_DELETE)
            .event(clone!(state => move |_: events::Click| {
                state.sidebar.base.delete_video();
            }))
        }))
    })
}
