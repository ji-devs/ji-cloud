use super::{
    sprite::dom::{
        render_sticker_sprite, render_sticker_sprite_raw, SpriteRawRenderOptions,
        SpriteRenderOptions,
    },
    state::*,
    text::dom::{
        render_sticker_text, render_sticker_text_raw, TextRawRenderOptions, TextRenderOptions,
    },
    video::dom::{
        render_sticker_video, render_sticker_video_raw, VideoRawRenderOptions, VideoRenderOptions,
    },
};
use dominator::{clone, html, Dom, DomBuilder};
use dominator_helpers::signals::EitherSignal;
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use shared::domain::jig::module::body::{Transform, _groups::design::Sticker as RawSticker};
use std::rc::Rc;
use utils::prelude::*;
use web_sys::HtmlElement;

pub fn mixin_sticker_button_signal(
    is_button: impl Signal<Item = bool> + 'static,
) -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    move |dom| {
        dom.style_signal(
            "cursor",
            is_button.map(|is_button| if is_button { "pointer" } else { "initial" }),
        )
        .style("user-select", "none")
        .style("-webkit-user-select", "none")
    }
}
pub fn mixin_sticker_button(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.style("cursor", "pointer")
        .style("user-select", "none")
        .style("-webkit-user-select", "none")
}

pub enum StickerRenderOptions {
    Sprite(SpriteRenderOptions),
    Text(TextRenderOptions),
    Video(VideoRenderOptions),
}

impl StickerRenderOptions {
    pub fn new(sticker: &Sticker, base: Option<BaseRenderOptions>) -> Self {
        match sticker {
            Sticker::Sprite(_) => Self::Sprite(SpriteRenderOptions {
                base: base.unwrap_or_default(),
            }),
            Sticker::Text(_) => Self::Text(TextRenderOptions {
                base: base.unwrap_or_default(),
            }),
            Sticker::Video(_) => Self::Video(VideoRenderOptions {
                base: base.unwrap_or_default(),
                ..Default::default()
            }),
        }
    }

    pub fn into_sprite_unchecked(self) -> SpriteRenderOptions {
        match self {
            Self::Sprite(inner) => inner,
            _ => panic!("not a sprite!"),
        }
    }
    pub fn into_text_unchecked(self) -> TextRenderOptions {
        match self {
            Self::Text(inner) => inner,
            _ => panic!("not a text!"),
        }
    }
    pub fn into_video_unchecked(self) -> VideoRenderOptions {
        match self {
            Self::Video(inner) => inner,
            _ => panic!("not a video!"),
        }
    }

    pub fn base(&self) -> &BaseRenderOptions {
        match self {
            Self::Sprite(inner) => &inner.base,
            Self::Text(inner) => &inner.base,
            Self::Video(inner) => &inner.base,
        }
    }
}

//Just a placeholder for backwards compatibility
#[derive(Default)]
pub struct BaseRenderOptions {}

pub enum StickerRawRenderOptions {
    Sprite(SpriteRawRenderOptions),
    Text(TextRawRenderOptions),
    Video(VideoRawRenderOptions),
}

impl StickerRawRenderOptions {
    pub fn new(sticker: &RawSticker, base: Option<BaseRawRenderOptions>) -> Self {
        match sticker {
            RawSticker::Sprite(_) => Self::Sprite(SpriteRawRenderOptions {
                base: base.unwrap_or_default(),
            }),
            RawSticker::Text(_) => Self::Text(TextRawRenderOptions {
                base: base.unwrap_or_default(),
            }),
            RawSticker::Video(_) => Self::Video(VideoRawRenderOptions {
                base: base.unwrap_or_default(),
                ..Default::default()
            }),
        }
    }

    pub fn into_sprite_unchecked(self) -> SpriteRawRenderOptions {
        match self {
            Self::Sprite(inner) => inner,
            _ => panic!("not a sprite!"),
        }
    }
    pub fn into_text_unchecked(self) -> TextRawRenderOptions {
        match self {
            Self::Text(inner) => inner,
            _ => panic!("not a text!"),
        }
    }
    pub fn into_video_unchecked(self) -> VideoRawRenderOptions {
        match self {
            Self::Video(inner) => inner,
            _ => panic!("not a video!"),
        }
    }

    pub fn base(&self) -> &BaseRawRenderOptions {
        match self {
            Self::Sprite(inner) => &inner.base,
            Self::Text(inner) => &inner.base,
            Self::Video(inner) => &inner.base,
        }
    }
}

#[derive(Default)]
pub struct BaseRawRenderOptions {
    //For sharing the size setting
    //if not supplied then it will just be created internally
    pub size: Option<Mutable<Option<(f64, f64)>>>,

    //For overriding the transform (read-only)
    //if not supplied, or its value is None, the original transform
    //will be used instead
    pub transform_override: Option<TransformOverride>,

    //For mixing in the sticker
    pub mixin: Option<Box<dyn Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,

    //The parent to use for containing the sticker
    //it's kinda weird, the sticker will append itself
    //but it's useful!
    pub parent: Option<DomBuilder<HtmlElement>>,
}

impl BaseRawRenderOptions {
    pub fn set_size(&mut self, size: Mutable<Option<(f64, f64)>>) {
        self.size = Some(size);
    }
    pub fn set_transform_override(&mut self, transform_override: TransformOverride) {
        self.transform_override = Some(transform_override);
    }
    pub fn set_mixin(
        &mut self,
        f: impl Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static,
    ) {
        self.mixin = Some(Box::new(f) as _);
    }

    pub fn set_parent(&mut self, parent: DomBuilder<HtmlElement>) {
        self.parent = Some(parent);
    }
}

#[derive(Clone)]
pub enum TransformOverride {
    Always(ReadOnlyMutable<Transform>),
    Sometimes(ReadOnlyMutable<Option<Transform>>),
}

impl TransformOverride {
    pub fn get_signal(&self, default: Transform) -> impl Signal<Item = Transform> {
        match self {
            Self::Always(t) => EitherSignal::Left(t.signal_cloned()),
            Self::Sometimes(t) => EitherSignal::Right(t.signal_cloned().map(
                clone!(default => move |transform_override| {
                    match transform_override {
                        Some(t) => t,
                        None => default.clone()
                    }
                }),
            )),
        }
    }
}

pub fn render_stickers<T: AsSticker>(stickers: Rc<Stickers<T>>) -> Dom {
    html!("empty-fragment", {
        .children_signal_vec(render_stickers_vec(stickers))
    })
}

pub fn render_stickers_vec<T: AsSticker>(stickers: Rc<Stickers<T>>) -> impl SignalVec<Item = Dom> {
    stickers
        .list
        .signal_vec_cloned()
        .enumerate()
        .map(clone!(stickers => move |(index, sticker)| {
            render_sticker(stickers.clone(), index, sticker, None)
        }))
}

pub fn render_sticker<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    sticker: T,
    opts: Option<StickerRenderOptions>,
) -> Dom {
    match sticker.as_ref() {
        Sticker::Sprite(sprite) => render_sticker_sprite(
            stickers,
            index,
            sprite.clone(),
            opts.map(|opts| opts.into_sprite_unchecked()),
        ),
        Sticker::Text(text) => render_sticker_text(
            stickers,
            index,
            text.clone(),
            opts.map(|opts| opts.into_text_unchecked()),
        ),
        Sticker::Video(video) => render_sticker_video(
            stickers,
            index,
            video.clone(),
            opts.map(|opts| opts.into_video_unchecked()),
        ),
    }
}

pub fn render_stickers_raw(stickers: &[RawSticker], theme_id: ThemeId) -> Dom {
    html!("empty-fragment", {
        .children(render_stickers_raw_vec(stickers, theme_id))
    })
}
pub fn render_stickers_raw_vec(stickers: &[RawSticker], theme_id: ThemeId) -> Vec<Dom> {
    stickers
        .iter()
        .map(|sticker| render_sticker_raw(sticker, theme_id, None))
        .collect::<Vec<Dom>>()
}

pub fn render_stickers_raw_mixin<F>(stickers: &[RawSticker], theme_id: ThemeId, mixin: F) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + Clone + 'static,
{
    html!("empty-fragment", {
        .children(render_stickers_raw_vec_mixin(stickers, theme_id, mixin))
    })
}

pub fn render_stickers_raw_vec_mixin<F>(
    stickers: &[RawSticker],
    theme_id: ThemeId,
    mixin: F,
) -> Vec<Dom>
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + Clone + 'static,
{
    stickers
        .iter()
        .map(|sticker| {
            let opts = match sticker {
                RawSticker::Sprite(_) => {
                    let mut opts = SpriteRawRenderOptions::default();
                    opts.base.set_mixin(mixin.clone());
                    StickerRawRenderOptions::Sprite(opts)
                }
                RawSticker::Text(_) => {
                    let mut opts = TextRawRenderOptions::default();
                    opts.base.set_mixin(mixin.clone());
                    StickerRawRenderOptions::Text(opts)
                }
                RawSticker::Video(_) => {
                    let mut opts = VideoRawRenderOptions::default();
                    opts.base.set_mixin(mixin.clone());
                    StickerRawRenderOptions::Video(opts)
                }
            };

            render_sticker_raw(sticker, theme_id, Some(opts))
        })
        .collect::<Vec<Dom>>()
}

pub fn render_sticker_raw(
    sticker: &RawSticker,
    theme_id: ThemeId,
    opts: Option<StickerRawRenderOptions>,
) -> Dom {
    match sticker {
        RawSticker::Sprite(sprite) => {
            render_sticker_sprite_raw(sprite, opts.map(|opts| opts.into_sprite_unchecked()))
        }
        RawSticker::Text(text) => {
            render_sticker_text_raw(text, theme_id, opts.map(|opts| opts.into_text_unchecked()))
        }
        RawSticker::Video(video) => {
            render_sticker_video_raw(video, opts.map(|opts| opts.into_video_unchecked()))
        }
    }
}
