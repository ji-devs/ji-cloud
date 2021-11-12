use super::state::Video;
use crate::base::styles;
use components::stickers::video::ext::*;
use dominator::{clone, html, with_node, Dom};
use shared::domain::jig::module::body::{
    _groups::design::YoutubeUrl, legacy::activity::VideoSource,
};
use std::rc::Rc;
use utils::prelude::*;

use web_sys::{HtmlElement, HtmlVideoElement};

use futures_signals::signal::SignalExt;

impl Video {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .class(&*styles::FULL_STAGE)
            .child({
                match &state.raw.src {
                    VideoSource::Youtube(yt) => {
                        state.clone().render_youtube(yt)
                    },
                    VideoSource::Direct(url) => {
                        state.clone().render_direct(url)
                    }
                }
            })
        })
    }

    pub fn render_direct(self: Rc<Self>, url: &str) -> Dom {
        let state = self;

        html!("video" => HtmlVideoElement, {
            .style("position", "absolute")
            .style_signal("transform", state.transform_signal())
            .style_signal("display", state.has_size_signal().map(|has_size| {
                if has_size { "block" } else { "none" }
            }))
            .style_signal("width", state.width_signal())
            .style_signal("height", state.height_signal())
            .style_signal("top", state.top_signal())
            .style_signal("left", state.left_signal())
            .style("transform-origin", {
                "center center"
            })
            .property("controls", false)
            .attribute("src", &state.base.activity_media_url(&url))
            .apply_if(state.raw.range.is_some(), |dom| {
                let (start, _) = state.raw.range.unwrap_ji();
                dom
                    .property("currentTime", start.round())
            })
            .event(clone!(state => move |_: events::LoadedMetadata| {
                state.start_gates.replace_with(|gates| {
                    gates.video = true;
                    *gates
                });

                state.video_size.set_neq(Some(
                    state.direct_api.borrow().as_ref().unwrap_ji().get_video_size()
                ));

            }))
            .event(clone!(state => move |_: events::Ended| {
                state.on_ended();
            }))
            .with_node!(elem => {
                .event(clone!(state => move |_evt: events::TimeUpdate| {
                    if let Some(end) = state.raw.range.map(|(_start, end)| end) {
                        let current_time = elem.current_time();
                        log::info!("{} vs. {}", current_time, end);

                        if current_time >= end {
                            state.on_ended();
                        }
                    }
                }))
            })
            .with_node!(_elem => {
                .future(clone!(state => async move {
                    state.first_play_signal().for_each(|first_play| {
                        if first_play {
                            state.direct_api.borrow().as_ref().unwrap_ji().play();
                        }
                        async {}
                    }).await;
                }))
            })
            .after_inserted(clone!(state => move |elem| {
                state.set_direct_api(elem);
            }))
        })
    }

    pub fn render_youtube(self: Rc<Self>, yt: &YoutubeUrl) -> Dom {
        let state = self;

        html!("video-youtube-player" => HtmlElement, {
            .apply_if(state.raw.range.is_some(), |dom| {
                let (start, end) = state.raw.range.unwrap_ji();
                dom
                    .property("start", start.round())
                    .property("end", end.round())
            })
            .property("videoId", yt.get_id())
            .property("hideControls", true)
            .style("position", "absolute")
            .style_signal("transform", state.transform_signal())
            .style_signal("display", state.has_size_signal().map(|has_size| {
                if has_size { "block" } else { "none" }
            }))
            .style_signal("width", state.width_signal())
            .style_signal("height", state.height_signal())
            .style_signal("top", state.top_signal())
            .style_signal("left", state.left_signal())
            .style("transform-origin", {
                "center center"
            })
            .property("captions", false)
            .property("muted", false)
            .property("loop", false)
            // always false since we imperatively play when the context is ready
            .property("autoplay", false)
            .event(clone!(state => move |_: events::Ready| {
                state.start_gates.replace_with(|gates| {
                    gates.video = true;
                    *gates
                });

                state.video_size.set_neq(Some(
                    state.yt_api.borrow().as_ref().unwrap_ji().get_video_size()
                ));

            }))
            .with_node!(_elem => {
                .future(clone!(state => async move {
                    state.first_play_signal().for_each(|first_play| {
                        if first_play {
                            state.yt_api.borrow().as_ref().unwrap_ji().play();
                        }
                        async {}
                    }).await;
                }))
            })
            .event(clone!(state => move |_: events::YoutubeEnded| {
                state.on_ended();
            }))
            .after_inserted(clone!(state => move |elem| {
                state.set_yt_api(elem);
            }))

        })
    }
}
