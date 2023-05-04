use super::{
    super::state::{AsSticker, Stickers},
    config::{YOUTUBE_EMBED_HEIGHT, YOUTUBE_EMBED_WIDTH},
    menu::dom::render_sticker_embed_menu,
    state::Embed,
    types::{
        GoogleSheetsEmbed, QuizletEmbed, SutoriEmbed, ThinglinkEmbed, VimeoEmbed, YoutubeEmbed,
    },
};
use crate::{
    stickers::{
        dom::{BaseRawRenderOptions, BaseRenderOptions},
        embed::types::{EmbedHost, ParseUrlExt},
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
use shared::domain::module::body::_groups::design::{
    DoneAction, Embed as RawEmbed, EmbedHost as RawEmbedHost, GoogleSheetId, QuizletId, SutoriId,
    ThinglinkId, VimeoUrl,
};
use std::rc::Rc;
use utils::{math::transform_signals, prelude::*};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

#[derive(Default)]
pub struct EmbedRenderOptions {
    pub base: BaseRenderOptions,
    pub on_ended: Option<Box<dyn Fn()>>,
}

#[derive(Default)]
pub struct EmbedRawRenderOptions {
    pub base: BaseRawRenderOptions,
    pub on_ended: Option<Rc<dyn Fn()>>,
}

fn render_youtube_embed(
    youtube: &Rc<YoutubeEmbed>,
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
        .prop_signal("videoId", youtube.url.signal_cloned().map(|url| {
            url.get_id().to_owned()
        }))
        // always autoplay since there's another layer for the play button
        .prop("autoplay", true)
        .prop_signal("captions", youtube.captions.signal())
        .prop_signal("muted", youtube.muted.signal())
        .prop_signal("loop", youtube.done_action.signal().map(|done_action| {
            matches!(done_action, Some(DoneAction::Loop))
        }))
        .prop_signal("start", youtube.start_at.signal())
        .prop_signal("end", youtube.end_at.signal())
        .apply(|dom| apply_transform(dom, &embed.transform))
        .event(clone!(embed => move |_: events::YoutubeEnded| {
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

fn render_vimeo_embed(
    vimeo: &Rc<VimeoEmbed>,
    embed: Rc<Embed>,
    _opts: Rc<EmbedRenderOptions>,
) -> Dom {
    html!("iframe", {
        .prop_signal("src", vimeo.url.signal_cloned().map(|url| {
            url.get_id().to_owned()
            // TODO: only the id?
        }))
        .apply(|dom| apply_transform(dom, &embed.transform))
    })
}

fn render_google_sheet_embed(
    google_sheet: &Rc<GoogleSheetsEmbed>,
    embed: Rc<Embed>,
    _opts: Rc<EmbedRenderOptions>,
) -> Dom {
    html!("iframe", {
        .prop_signal("src", google_sheet.url.signal_cloned().map(|url| {
            url.get_id().to_owned()
            // TODO: only the id?
        }))
        .apply(|dom| apply_transform(dom, &embed.transform))
    })
}

fn render_thinglink_embed(
    thinglink: &Rc<ThinglinkEmbed>,
    embed: Rc<Embed>,
    _opts: Rc<EmbedRenderOptions>,
) -> Dom {
    html!("iframe", {
        .prop_signal("src", thinglink.url.signal_cloned().map(|url| {
            url.get_id().to_owned()
            // TODO: only the id?
            // frameborder="0" allowfullscreen
        }))
        .apply(|dom| apply_transform(dom, &embed.transform))
    })
}

fn render_quizlet_embed(
    quizlet: &Rc<QuizletEmbed>,
    embed: Rc<Embed>,
    _opts: Rc<EmbedRenderOptions>,
) -> Dom {
    html!("iframe", {
        .prop_signal("src", quizlet.url.signal_cloned().map(|url| {
            url.get_id().to_owned()
            // TODO: only the id?
            // frameborder="0" allowfullscreen
        }))
        .apply(|dom| apply_transform(dom, &embed.transform))
    })
}

fn render_sutori_embed(
    sutori: &Rc<SutoriEmbed>,
    embed: Rc<Embed>,
    _opts: Rc<EmbedRenderOptions>,
) -> Dom {
    html!("iframe", {
        .prop_signal("src", sutori.url.signal_cloned().map(|url| {
            url.get_id().to_owned()
            // TODO: only the id?
            // frameborder="0" allowfullscreen
        }))
        .apply(|dom| apply_transform(dom, &embed.transform))
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
                                EmbedHost::Vimeo(vimeo) => Some(render_vimeo_embed(&vimeo, Rc::clone(&embed), Rc::clone(&opts))),
                                EmbedHost::GoogleSheet(google_sheet) => Some(render_google_sheet_embed(&google_sheet, Rc::clone(&embed), Rc::clone(&opts))),
                                EmbedHost::Edpuzzle(_) => todo!(),
                                EmbedHost::Puzzel(_) => todo!(),
                                EmbedHost::Quizlet(quizlet) => Some(render_quizlet_embed(&quizlet, Rc::clone(&embed), Rc::clone(&opts))),
                                EmbedHost::Thinglink(thinglink) => Some(render_thinglink_embed(&thinglink, Rc::clone(&embed), Rc::clone(&opts))),
                                EmbedHost::Sutori(sutori) => Some(render_sutori_embed(&sutori, Rc::clone(&embed), Rc::clone(&opts))),
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
                                        .prop_signal("videoId", youtube.url.signal_cloned().map(|url| {
                                            url.get_id().to_owned()
                                        }))
                                        .apply(|dom| apply_transform(dom, &embed.transform))
                                    })
                                ),
                                EmbedHost::Vimeo(vimeo) => Some(
                                    html!("iframe", {
                                        .prop_signal("src", vimeo.url.signal_ref(|url| {
                                            get_vimeo_url(&url)
                                        }))
                                        .apply(|dom| apply_transform(dom, &embed.transform))
                                    })
                                ),
                                EmbedHost::GoogleSheet(google_sheet) => Some(
                                    html!("iframe", {
                                        .prop_signal("src", google_sheet.url.signal_ref(|url| {
                                            get_google_sheet_url(&url)
                                        }))
                                        .apply(|dom| apply_transform(dom, &embed.transform))
                                    })
                                ),
                                EmbedHost::Edpuzzle(_) => todo!(),
                                EmbedHost::Puzzel(_) => todo!(),
                                EmbedHost::Quizlet(quizlet) => Some(
                                    html!("iframe", {
                                        .prop_signal("src", quizlet.url.signal_ref(|url| {
                                            get_quizlet_url(&url)
                                        }))
                                        .apply(|dom| apply_transform(dom, &embed.transform))
                                    })
                                ),
                                EmbedHost::Thinglink(thinglink) => Some(
                                    html!("iframe", {
                                        .prop_signal("src", thinglink.url.signal_ref(|url| {
                                            get_thinglink_url(&url)
                                        }))
                                        // frameborder="0" allowfullscreen
                                        .apply(|dom| apply_transform(dom, &embed.transform))
                                    })
                                ),
                                EmbedHost::Sutori(sutori) => Some(
                                    html!("iframe", {
                                        .prop_signal("src", sutori.url.signal_ref(|url| {
                                            get_sutori_url(&url)
                                        }))
                                        // frameborder="0" allowfullscreen
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
                RawEmbedHost::Youtube(youtube) => {
                    html!("video-youtube-player" => HtmlElement, {
                        .prop("videoId", youtube.url.get_id())
                        .prop("autoplay", youtube.autoplay)
                        .prop("loop", matches!(youtube.done_action, Some(DoneAction::Loop)))
                        .prop("captions", youtube.captions)
                        .prop("muted", youtube.muted)
                        .apply(|mut dom| {
                            if let Some(start_at) = youtube.start_at {
                                dom = dom.prop("start", start_at);
                            }
                            if let Some(end_at) = youtube.end_at {
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
                RawEmbedHost::Vimeo(vimeo) => {
                    html!("iframe", {
                        .prop("src", get_vimeo_url(&vimeo.url))
                        .style("display", "block")
                        .style("width", "100%")
                        .style("height", "100%")
                        // frameborder="0" allow="autoplay; fullscreen; picture-in-picture" allowfullscreen
                    })
                }
                RawEmbedHost::GoogleSheet(google_sheet) => {
                    html!("iframe", {
                        .prop("src", get_google_sheet_url(&google_sheet.url))
                        .style("display", "block")
                        .style("width", "100%")
                        .style("height", "100%")
                    })
                }
                RawEmbedHost::Edpuzzle(_) => todo!(),
                RawEmbedHost::Puzzel(_) => todo!(),
                RawEmbedHost::Quizlet(quizlet) => {
                    html!("iframe", {
                        .prop("src", get_quizlet_url(&quizlet.url))
                        .style("display", "block")
                        .style("width", "100%")
                        .style("height", "100%")
                    })
                }
                RawEmbedHost::Thinglink(thinglink) => {
                    html!("iframe", {
                        .prop("src", get_thinglink_url(&thinglink.url))
                        .style("display", "block")
                        .style("width", "100%")
                        .style("height", "100%")
                        // frameborder="0" allowfullscreen
                    })
                }
                RawEmbedHost::Sutori(sutori) => {
                    html!("iframe", {
                        .prop("src", get_sutori_url(&sutori.url))
                        .style("display", "block")
                        .style("width", "100%")
                        .style("height", "100%")
                    })
                }
            }
        })
        .apply_if(mixin.is_some(), move |dom| dom.apply(mixin.unwrap_ji()))
        .into_dom()
}

fn get_google_sheet_url(google_sheet: &GoogleSheetId) -> String {
    format!(
        "https://docs.google.com/spreadsheets/d/e/{}/pubhtml?widget=true&amp;headers=false",
        google_sheet.get_id()
    )
}

fn get_thinglink_url(thinglink: &ThinglinkId) -> String {
    format!("https://www.thinglink.com/card/{}", thinglink.get_id())
}

fn get_quizlet_url(quizlet: &QuizletId) -> String {
    format!("https://quizlet.com/{}/flashcards/embed", quizlet.get_id())
}

fn get_sutori_url(sutori: &SutoriId) -> String {
    format!("https://www.sutori.com/en/story/{}/embed", sutori.get_id())
}

fn get_vimeo_url(vimeo: &VimeoUrl) -> String {
    format!("https://player.vimeo.com/video/{}", vimeo.get_id())
}
