use super::{
    super::state::{AsSticker, Stickers},
    config::{YOUTUBE_VIDEO_HEIGHT, YOUTUBE_VIDEO_WIDTH},
    menu::dom::render_sticker_video_menu,
    state::Video,
};
use crate::{
    stickers::{
        dom::{BaseRawRenderOptions, BaseRenderOptions},
        video::ext::YoutubeUrlExt,
    },
    transform::{
        dom::render_transform,
        state::{ResizeLevel, TransformState},
    },
};
use dominator::{clone, html, with_node, Dom, DomBuilder};
use dominator_helpers::signals::DefaultSignal;
use futures_signals::signal::{Mutable, ReadOnlyMutable, SignalExt};
use gloo_timers::future::TimeoutFuture;
use js_sys::Reflect;
use shared::domain::jig::module::body::{
    _groups::design::{Video as RawVideo, VideoHost, YoutubeUrl},
    video::DoneAction,
};
use std::rc::Rc;
use utils::{math::transform_signals, prelude::*};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

pub struct VideoRenderOptions {
    pub base: BaseRenderOptions,
    pub captions: ReadOnlyMutable<bool>,
    pub muted: ReadOnlyMutable<bool>,
    pub done_action: ReadOnlyMutable<Option<DoneAction>>,
    pub on_ended: Option<Box<dyn Fn()>>,
}

impl Default for VideoRenderOptions {
    fn default() -> Self {
        let read_only_bool = Mutable::new(false).read_only();
        Self {
            base: BaseRenderOptions::default(),
            captions: read_only_bool.clone(),
            muted: read_only_bool.clone(),
            done_action: Mutable::new(None).read_only(),
            on_ended: None,
        }
    }
}

#[derive(Default)]
pub struct VideoRawRenderOptions {
    pub base: BaseRawRenderOptions,
    pub captions: Mutable<bool>,
    pub muted: Mutable<bool>,
    pub autoplay: Mutable<bool>,
    pub _loop: Mutable<bool>,
    pub on_ended: Option<Rc<dyn Fn()>>,
}

fn render_youtube_video(
    youtube: &YoutubeUrl,
    video: Rc<Video>,
    opts: Rc<VideoRenderOptions>,
) -> Dom {
    html!("video-youtube-player" => HtmlElement, {
        .with_node!(elem => {
            .future(clone!(elem, video => async move {
                video.is_playing.signal().for_each(|is_playing| {
                    if is_playing {
                        let play_method = Reflect::get(
                            &elem,
                            &JsValue::from_str("play")
                        )
                            .unwrap();

                        let play_method = play_method.dyn_ref::<js_sys::Function>().unwrap();
                        let _ = play_method.call0(&elem);
                    }
                    async {}
                }).await;
            }))
        })
        .style_signal("display", video.is_playing.signal().map(|is_playing| {
            match is_playing {
                true => "block",
                false => "none",
            }
        }))
        .property("videoId", youtube.get_id())
        // always autoplay since there's another layer for the play button
        .property("autoplay", true)
        .property_signal("captions", opts.captions.signal())
        .property_signal("muted", opts.muted.signal())
        .property_signal("loop", opts.done_action.signal().map(|done_action| {
            matches!(done_action, Some(DoneAction::Loop))
        }))
        .apply(|dom| apply_transform(dom, &video.transform))
        .event(clone!(video, opts => move |_: events::YoutubeEnded| {
            video.is_playing.set_neq(false);
            if let Some(on_ended) = opts.on_ended.as_ref() {
                (on_ended) ();
            }
        }))
        .event(clone!(video => move |evt: events::YoutubePaused| {
            spawn_local(clone!(video => async move {
                // wait for half a second and then check if still paused, if still paused than show the overlay again

                TimeoutFuture::new(300).await;

                let target = evt.dyn_target::<HtmlElement>().unwrap();
                let player_state = Reflect::get(
                    &target,
                    &JsValue::from_str("playerState")
                )
                    .unwrap_ji()
                    .as_string()
                    .unwrap_ji();

                log::info!("{:?}", player_state);
                if player_state == "paused" {
                    video.is_playing.set_neq(false);
                }
            }));

        }))
        .event(clone!(video => move |_: events::YoutubePlaying| {
            video.is_playing.set_neq(true);

        }))
    })
}

fn apply_transform<A: AsRef<HtmlElement>>(
    dom: DomBuilder<A>,
    transform: &TransformState,
) -> DomBuilder<A> {
    dom.style("position", "absolute")
        .style_signal("transform", transform.rotation_matrix_string_signal())
        .style_signal("top", transform.y_px_signal().map(|x| format!("{}px", x)))
        .style_signal("left", transform.x_px_signal().map(|x| format!("{}px", x)))
        .style_signal(
            "width",
            transform.width_px_signal().map(|x| format!("{}px", x)),
        )
        .style_signal(
            "height",
            transform.height_px_signal().map(|x| format!("{}px", x)),
        )
}

pub fn render_sticker_video<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    video: Rc<Video>,
    opts: Option<VideoRenderOptions>,
) -> Dom {
    let opts = Rc::new(opts.unwrap_or_default());

    html!("document-fragment", {
        .child_signal(video.playing_started.signal().map(clone!(video => move |playing_started| {
            match playing_started {
                false => None,
                true => {
                    Some(html!("document-fragment", {
                        .child_signal(video.host.signal_cloned().map(clone!(video, opts => move|host| {
                            match host {
                                VideoHost::Youtube(youtube) => Some(render_youtube_video(&youtube, Rc::clone(&video), Rc::clone(&opts))),
                            }
                        })))
                    }))
                },
            }
        })))
        .child_signal(video.is_playing.signal().map(clone!(video => move |is_playing| {
            match is_playing {
                true => None,
                false => {
                    Some(html!("document-fragment", {
                        .child_signal(video.host.signal_cloned().map(clone!(video => move|host| {
                            match host {
                                VideoHost::Youtube(youtube) => Some(
                                    html!("video-youtube-thumbnail", {
                                        .property("videoId", youtube.get_id())
                                        .apply(|dom| apply_transform(dom, &video.transform))
                                    })
                                ),
                            }
                        })))
                        .children(&mut [
                            render_transform(
                                video.transform.clone(),
                                ResizeLevel::KeepAspectRatio,
                                Some(clone!(stickers, index, video => move || render_sticker_video_menu(stickers.clone(), index.clone(), Rc::clone(&video))))
                            ),
                            html!("div", {
                                .apply(|dom| apply_transform(dom, &video.transform))
                                .style("display", "inline-grid")
                                .style("place-content", "center")
                                .style("pointer-events", "none")
                                .style("position", "relative")
                                .style("z-index", "1")
                                .child(html!("video-youtube-play-button", {
                                    .event(clone!(video => move|_: events::Click| {
                                        video.playing_started.set_neq(true);
                                        video.is_playing.set(true);
                                    }))
                                }))
                            })
                        ])
                    }))
                },
            }
        })))

    })
}

pub fn render_sticker_video_raw(video: &RawVideo, opts: Option<VideoRawRenderOptions>) -> Dom {
    const COORDS_IN_CENTER: bool = true;

    let opts = opts.unwrap_or_default();

    let parent = opts
        .base
        .parent
        .unwrap_or_else(|| DomBuilder::new_html("empty-fragment"));

    let size = opts.base.size.unwrap_or_else(|| Mutable::new(None));
    size.set(Some((YOUTUBE_VIDEO_WIDTH, YOUTUBE_VIDEO_HEIGHT)));

    let transform = video.transform.clone();

    let transform_override = opts.base.transform_override;

    let get_transform_signal = clone!(transform, transform_override => move || {
        DefaultSignal::new(
            transform.clone(),
            transform_override.clone().map(clone!(transform => move |t| t.get_signal(transform)))
        )
    });

    let x_signal = transform_signals::x_px(
        COORDS_IN_CENTER,
        get_transform_signal(),
        size.signal_cloned(),
    );
    let y_signal = transform_signals::y_px(
        COORDS_IN_CENTER,
        get_transform_signal(),
        size.signal_cloned(),
    );
    let width_signal = transform_signals::width_px(
        COORDS_IN_CENTER,
        get_transform_signal(),
        size.signal_cloned(),
    );
    let height_signal = transform_signals::height_px(
        COORDS_IN_CENTER,
        get_transform_signal(),
        size.signal_cloned(),
    );

    let mixin = opts.base.mixin;

    let on_ended = opts.on_ended;
    parent
        .style("position", "absolute")
        .style_signal(
            "transform",
            get_transform_signal().map(|t| t.rotation_matrix_string()),
        )
        .style_signal("left", x_signal.map(|x| format!("{}px", x)))
        .style_signal("top", y_signal.map(|x| format!("{}px", x)))
        .style_signal("width", width_signal.map(|x| format!("{}px", x)))
        .style_signal("height", height_signal.map(|x| format!("{}px", x)))
        .child({
            match &video.host {
                VideoHost::Youtube(youtube_url) => {
                    html!("video-youtube-player" => HtmlElement, {
                        .property("videoId", youtube_url.get_id())
                        .property_signal("autoplay", opts.autoplay.signal())
                        .property_signal("loop", opts._loop.signal())
                        .property_signal("captions", opts.captions.signal())
                        .property_signal("muted", opts.muted.signal())

                        .style("display", "block")
                        .style("width", "100%")
                        .style("height", "100%")
                        .event(clone!(on_ended => move |_: events::YoutubeEnded| {
                            if let Some(on_ended) = on_ended.as_ref() {
                                (on_ended) ();
                            }
                        }))
                    })
                }
            }
        })
        .apply_if(mixin.is_some(), move |dom| dom.apply(mixin.unwrap_ji()))
        .into_dom()
}
