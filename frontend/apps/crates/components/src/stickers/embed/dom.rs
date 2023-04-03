use super::{
    super::state::{AsSticker, Stickers},
    config::{YOUTUBE_EMBED_HEIGHT, YOUTUBE_EMBED_WIDTH},
    menu::dom::render_sticker_embed_menu,
    state::Embed,
};
use crate::{
    stickers::{
        dom::{BaseRawRenderOptions, BaseRenderOptions},
        embed::ext::YoutubeUrlExt,
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
use shared::domain::module::body::{
    _groups::design::{Embed as RawEmbed, EmbedHost, YoutubeEmbed},
    video::DoneAction,
};
use std::rc::Rc;
use utils::{math::transform_signals, prelude::*};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

pub struct EmbedRenderOptions {
    pub base: BaseRenderOptions,
    pub captions: ReadOnlyMutable<bool>,
    pub muted: ReadOnlyMutable<bool>,
    pub done_action: ReadOnlyMutable<Option<DoneAction>>,
    pub on_ended: Option<Box<dyn Fn()>>,
}

impl Default for EmbedRenderOptions {
    fn default() -> Self {
        let read_only_bool = Mutable::new(false).read_only();
        Self {
            base: BaseRenderOptions::default(),
            captions: read_only_bool.clone(),
            muted: read_only_bool,
            done_action: Mutable::new(None).read_only(),
            on_ended: None,
        }
    }
}

#[derive(Default)]
pub struct EmbedRawRenderOptions {
    pub base: BaseRawRenderOptions,
    pub captions: Mutable<bool>,
    pub muted: Mutable<bool>,
    pub autoplay: Mutable<bool>,
    pub _loop: Mutable<bool>,
    pub on_ended: Option<Rc<dyn Fn()>>,
}

fn render_youtube_embed(
    youtube: &YoutubeEmbed,
    embed: Rc<Embed>,
    opts: Rc<EmbedRenderOptions>,
) -> Dom {
    html!("video-youtube-player" => HtmlElement, {
        .with_node!(elem => {
            .future(clone!(elem, embed => async move {
                embed.is_playing.signal().for_each(|is_playing| {
                    if is_playing {
                        let play_method = Reflect::get(
                            &elem,
                            &JsValue::from_str("play")
                        )
                            .unwrap_ji();

                        let play_method = play_method.dyn_ref::<js_sys::Function>().unwrap_ji();
                        let _ = play_method.call0(&elem);
                    }
                    async {}
                }).await;
            }))
        })
        .style_signal("display", embed.is_playing.signal().map(|is_playing| {
            match is_playing {
                true => "block",
                false => "none",
            }
        }))
        .prop("videoId", youtube.url.get_id())
        // always autoplay since there's another layer for the play button
        .prop("autoplay", true)
        .prop_signal("captions", opts.captions.signal())
        .prop_signal("muted", opts.muted.signal())
        .prop_signal("loop", opts.done_action.signal().map(|done_action| {
            matches!(done_action, Some(DoneAction::Loop))
        }))
        .prop_signal("start", embed.start_at.signal())
        .prop_signal("end", embed.end_at.signal())
        .apply(|dom| apply_transform(dom, &embed.transform))
        .event(clone!(embed, opts => move |_: events::YoutubeEnded| {
            embed.is_playing.set_neq(false);
            if let Some(on_ended) = opts.on_ended.as_ref() {
                (on_ended) ();
            }
        }))
        .event(clone!(embed => move |evt: events::YoutubePaused| {
            spawn_local(clone!(embed => async move {
                // wait for half a second and then check if still paused, if still paused than show the overlay again

                TimeoutFuture::new(300).await;

                let target = evt.dyn_target::<HtmlElement>().unwrap_ji();
                let player_state = Reflect::get(
                    &target,
                    &JsValue::from_str("playerState")
                )
                    .unwrap_ji()
                    .as_string()
                    .unwrap_ji();

                log::info!("{:?}", player_state);
                if player_state == "paused" {
                    embed.is_playing.set_neq(false);
                }
            }));

        }))
        .event(clone!(embed => move |_: events::YoutubePlaying| {
            embed.is_playing.set_neq(true);

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

pub fn render_sticker_embed<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    embed: Rc<Embed>,
    opts: Option<EmbedRenderOptions>,
) -> Dom {
    let opts = Rc::new(opts.unwrap_or_default());

    html!("document-fragment", {
        .child_signal(embed.playing_started.signal().map(clone!(embed => move |playing_started| {
            match playing_started {
                false => None,
                true => {
                    Some(html!("document-fragment", {
                        .child_signal(embed.host.signal_cloned().map(clone!(embed, opts => move|host| {
                            match host {
                                EmbedHost::Youtube(youtube) => Some(render_youtube_embed(&youtube, Rc::clone(&embed), Rc::clone(&opts))),
                            }
                        })))
                    }))
                },
            }
        })))
        .child_signal(embed.is_playing.signal().map(clone!(embed => move |is_playing| {
            match is_playing {
                true => None,
                false => {
                    Some(html!("document-fragment", {
                        .child_signal(embed.host.signal_cloned().map(clone!(embed => move|host| {
                            match host {
                                EmbedHost::Youtube(youtube) => Some(
                                    html!("video-youtube-thumbnail", {
                                        .prop("videoId", youtube.url.get_id())
                                        .apply(|dom| apply_transform(dom, &embed.transform))
                                    })
                                ),
                            }
                        })))
                        .children(&mut [
                            render_transform(
                                embed.transform.clone(),
                                ResizeLevel::KeepAspectRatio,
                                Some(clone!(stickers, index, embed => move || render_sticker_embed_menu(stickers.clone(), index.clone(), Rc::clone(&embed))))
                            ),
                            html!("div", {
                                .apply(|dom| apply_transform(dom, &embed.transform))
                                .style("display", "inline-grid")
                                .style("place-content", "center")
                                .style("pointer-events", "none")
                                .style("position", "relative")
                                .style("z-index", "1")
                                .child(html!("video-youtube-play-button", {
                                    .event(clone!(embed => move|_: events::Click| {
                                        embed.playing_started.set_neq(true);
                                        embed.is_playing.set(true);
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

pub fn render_sticker_embed_raw(embed: &RawEmbed, opts: Option<EmbedRawRenderOptions>) -> Dom {
    const COORDS_IN_CENTER: bool = true;

    let opts = opts.unwrap_or_default();

    let parent = opts
        .base
        .parent
        .unwrap_or_else(|| DomBuilder::new_html("empty-fragment"));

    let size = opts.base.size.unwrap_or_else(|| Mutable::new(None));
    size.set(Some((YOUTUBE_EMBED_WIDTH, YOUTUBE_EMBED_HEIGHT)));

    let transform = embed.transform.clone();

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
            match &embed.host {
                EmbedHost::Youtube(youtube_embed) => {
                    html!("video-youtube-player" => HtmlElement, {
                        .prop("videoId", youtube_embed.url.get_id())
                        .prop_signal("autoplay", opts.autoplay.signal())
                        .prop_signal("loop", opts._loop.signal())
                        .prop_signal("captions", opts.captions.signal())
                        .prop_signal("muted", opts.muted.signal())
                        .apply(|mut dom| {
                            if let Some(start_at) = embed.start_at {
                                dom = dom.prop("start", start_at);
                            }
                            if let Some(end_at) = embed.end_at {
                                dom = dom.prop("end", end_at);
                            }
                            dom
                        })
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
