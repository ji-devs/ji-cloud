use super::{
    embed::dom::{
        render_sticker_embed, render_sticker_embed_raw, EmbedRawRenderOptions, EmbedRenderOptions,
    },
    sprite::dom::{
        render_sticker_sprite, render_sticker_sprite_raw, SpriteRawRenderOptions,
        SpriteRenderOptions,
    },
    state::*,
    text::dom::{
        render_sticker_text, render_sticker_text_raw, TextRawRenderOptions, TextRenderOptions,
    },
};
use dominator::{clone, html, Dom, DomBuilder};
use dominator_helpers::signals::EitherSignal;
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use shared::domain::module::body::{
    HoverAnimation, StickerHidden, Transform, _groups::design::Sticker as RawSticker,
};
use std::rc::Rc;
use utils::{keyboard::KeyEvent, prelude::*};
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

#[derive(Clone)]
pub enum StickerRenderOptions {
    Sprite(Rc<SpriteRenderOptions>),
    Text(Rc<TextRenderOptions>),
    Embed(Rc<EmbedRenderOptions>),
}

impl StickerRenderOptions {
    pub fn new(sticker: &Sticker, base: Option<BaseRenderOptions>) -> Self {
        match sticker {
            Sticker::Sprite(_) => Self::Sprite(Rc::new(SpriteRenderOptions {
                base: base.unwrap_or_default(),
            })),
            Sticker::Text(_) => Self::Text(Rc::new(TextRenderOptions {
                base: base.unwrap_or_default(),
            })),
            Sticker::Embed(_) => Self::Embed(Rc::new(EmbedRenderOptions {
                base: base.unwrap_or_default(),
                ..Default::default()
            })),
        }
    }

    pub fn into_sprite_unchecked(self) -> Rc<SpriteRenderOptions> {
        match self {
            Self::Sprite(inner) => inner,
            _ => panic!("not a sprite!"),
        }
    }
    pub fn into_text_unchecked(self) -> Rc<TextRenderOptions> {
        match self {
            Self::Text(inner) => inner,
            _ => panic!("not a text!"),
        }
    }
    pub fn into_embed_unchecked(self) -> Rc<EmbedRenderOptions> {
        match self {
            Self::Embed(inner) => inner,
            _ => panic!("not an embed!"),
        }
    }

    pub fn base(&self) -> &BaseRenderOptions {
        match self {
            Self::Sprite(inner) => &inner.base,
            Self::Text(inner) => &inner.base,
            Self::Embed(inner) => &inner.base,
        }
    }
}

//Just a placeholder for backwards compatibility
#[derive(Default, Clone)]
pub struct BaseRenderOptions {
    pub animations: bool,
}

impl BaseRenderOptions {
    pub fn new_animations() -> Self {
        Self { animations: true }
    }
}

pub enum StickerRawRenderOptions {
    Sprite(SpriteRawRenderOptions),
    Text(TextRawRenderOptions),
    Embed(EmbedRawRenderOptions),
}

impl StickerRawRenderOptions {
    pub fn new(sticker: &RawSticker, base: Option<BaseRawRenderOptions>) -> Self {
        match sticker {
            RawSticker::Sprite(_) => Self::Sprite(SpriteRawRenderOptions {
                base: base.unwrap_or_default(),
            }),
            RawSticker::Text(_) => Self::Text(TextRawRenderOptions {
                base: base.unwrap_or_default(),
                measurer_mixin: None,
            }),
            RawSticker::Embed(_) => Self::Embed(EmbedRawRenderOptions {
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
    pub fn into_embed_unchecked(self) -> EmbedRawRenderOptions {
        match self {
            Self::Embed(inner) => inner,
            _ => panic!("not an embed!"),
        }
    }

    pub fn base(&self) -> &BaseRawRenderOptions {
        match self {
            Self::Sprite(inner) => &inner.base,
            Self::Text(inner) => &inner.base,
            Self::Embed(inner) => &inner.base,
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

pub fn render_stickers_options<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    opts: BaseRenderOptions,
) -> Dom {
    html!("empty-fragment", {
        .children_signal_vec(render_stickers_vec_options(stickers, opts))
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

pub fn render_stickers_vec_options<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    opts: BaseRenderOptions,
) -> impl SignalVec<Item = Dom> {
    stickers
        .list
        .signal_vec_cloned()
        .enumerate()
        .map(clone!(stickers => move |(index, sticker)| {
            let opts = StickerRenderOptions::new(sticker.as_ref(), Some(opts.clone()));
            render_sticker(stickers.clone(), index, sticker, Some(opts))
        }))
}

pub fn render_sticker<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    sticker: T,
    opts: Option<StickerRenderOptions>,
) -> Dom {
    html!("empty-fragment", {
        .global_event(clone!(stickers, index => move |evt:events::KeyDown| {
            if KeyEvent::from(evt).is_delete_key() {
                if let Some(selected) = stickers.selected_index.get_cloned() {
                    if Some(selected) == index.get_cloned() {
                        let can_delete = if let Some(text) = stickers.get_as_text(selected) {
                            !text.is_editing.get() && text.can_delete.get()
                        } else {
                            true
                        };

                        if can_delete {
                            // If we don't deselect the currently selected sticker, then this event will
                            // trigger on each sticker which moves into this index after the current one is
                            // deleted.
                            stickers.selected_index.set(None);
                            stickers.delete_index(selected);
                        }
                    }
                }
            }
        }))
        .child(
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
                Sticker::Embed(embed) => render_sticker_embed(
                    stickers,
                    index,
                    embed.clone(),
                    opts.map(|opts| opts.into_embed_unchecked()),
                ),
            }
        )
    })
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
                RawSticker::Embed(_) => {
                    let mut opts = EmbedRawRenderOptions::default();
                    opts.base.set_mixin(mixin.clone());
                    StickerRawRenderOptions::Embed(opts)
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
        RawSticker::Embed(embed) => {
            render_sticker_embed_raw(embed, opts.map(|opts| opts.into_embed_unchecked()))
        }
    }
}

pub fn sticker_animation(
    animation: Option<HoverAnimation>,
    slot: impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
) -> Dom {
    html!("animations-hover", {
        .apply(|dom| {
            match &animation {
                None => dom,
                Some(effect) => dom.class(effect.as_str()),
            }

        })
        .apply(|dom| {
            slot(dom)
        })
    })
}

pub fn sticker_hidden(
    hidden: &Option<StickerHidden>,
    slot: impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
) -> Dom {
    html!("animations-hide", {
        .apply(|dom| {
            match &hidden {
                None => dom,
                Some(hidden) => {
                    let visible = Mutable::new(hidden.is_on_click());
                    let dom = dom
                        .prop_signal("visible", visible.signal())
                        .event(move |_: events::Click| {
                            visible.replace_with(|visible| !*visible);
                        });
                    match hidden {
                        StickerHidden::OnClick(effect) => {
                            dom.prop("effect", effect.as_str())
                        },
                        StickerHidden::UntilClick(effect) => {
                            dom.prop("effect", effect.as_str())
                        },
                    }
                },
            }
        })
        .apply(|dom| {
            slot(dom)
        })
    })
}
