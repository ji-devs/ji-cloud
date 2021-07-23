use dominator::{Dom, DomBuilder, clone, html};
use dominator_helpers::signals::EitherSignal;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{ReadOnlyMutable, Mutable, Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use super::{
    state::*,
    sprite::dom::{
        SpriteRawRenderOptions,
        render_sticker_sprite, 
        render_sticker_sprite_raw, 
    }, 
    text::dom::{
        TextRawRenderOptions,
        render_sticker_text, 
        render_sticker_text_raw, 
    }
};
use web_sys::HtmlElement;
use shared::domain::jig::module::body::{Transform, _groups::design::Sticker as RawSticker};

pub fn mixin_sticker_button(dom:DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom
        .style("cursor", "pointer")
        .style("user-select", "none")
        .style("-webkit-user-select", "none")
}

pub enum StickerRawRenderOptions {
    Sprite(SpriteRawRenderOptions),
    Text(TextRawRenderOptions),
}

impl StickerRawRenderOptions {
    pub fn into_sprite_unchecked(self) -> SpriteRawRenderOptions {
        match self {
            Self::Sprite(inner) => inner,
            _ => panic!("not a sprite!")
        }
    }
    pub fn into_text_unchecked(self) -> TextRawRenderOptions {
        match self {
            Self::Text(inner) => inner,
            _ => panic!("not a text!")
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
    pub parent: Option<DomBuilder<HtmlElement>>
}

impl BaseRawRenderOptions {
    pub fn set_size(&mut self, size: Mutable<Option<(f64, f64)>>) {
        self.size = Some(size);
    }
    pub fn set_transform_override(&mut self, transform_override: TransformOverride) {
        self.transform_override = Some(transform_override);
    }
    pub fn set_mixin(&mut self, f: impl Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static) {
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
            Self::Always(t) => {
                EitherSignal::Left(t.signal_cloned())
            },
            Self::Sometimes(t) => {
                EitherSignal::Right(t.signal_cloned()
                    .map(clone!(default => move |transform_override| {
                        match transform_override {
                            Some(t) => t,
                            None => default.clone()
                        }
                    }))
                )
            }
        }
    }
}

pub fn render_stickers<T: AsSticker>(stickers:Rc<Stickers<T>>) -> Dom {
    html!("empty-fragment", {
        .children_signal_vec(render_stickers_vec(stickers))
    })
}
pub fn render_stickers_vec<T: AsSticker>(stickers:Rc<Stickers<T>>) -> impl SignalVec<Item = Dom> {
    stickers.list
        .signal_vec_cloned()
        .enumerate()
        .map(clone!(stickers => move |(index, sticker)| {
            match sticker.as_ref() {
                Sticker::Sprite(sprite) => render_sticker_sprite(stickers.clone(), index, sprite.clone()),
                Sticker::Text(text) => render_sticker_text(stickers.clone(), index, text.clone()),
            }
        }))
}

pub fn render_stickers_raw(stickers:&[RawSticker], theme_id: ThemeId) -> Dom {
    html!("empty-fragment", {
        .children(render_stickers_raw_vec(stickers, theme_id))
    })
}
pub fn render_stickers_raw_vec(stickers:&[RawSticker], theme_id: ThemeId) -> Vec<Dom> {
    stickers
        .iter()
        .map(|sticker| render_sticker_raw(&sticker, theme_id, None))
        .collect::<Vec<Dom>>()
}

pub fn render_stickers_raw_mixin<F>(stickers:&[RawSticker], theme_id: ThemeId, mixin: F) -> Dom 
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + Clone + 'static
{
    html!("empty-fragment", {
        .children(render_stickers_raw_vec_mixin(stickers, theme_id, mixin))
    })
}

pub fn render_stickers_raw_vec_mixin<F>(stickers:&[RawSticker], theme_id: ThemeId, mixin: F) -> Vec<Dom>
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + Clone + 'static
{


    stickers
        .iter()
        .map(|sticker| {
            let opts = match sticker {
                RawSticker::Sprite(_) => {
                    let mut opts = SpriteRawRenderOptions::default();
                    opts.base.set_mixin(mixin.clone());
                    StickerRawRenderOptions::Sprite(opts)
                },
                RawSticker::Text(_) => {
                    let mut opts = TextRawRenderOptions::default();
                    opts.base.set_mixin(mixin.clone());
                    StickerRawRenderOptions::Text(opts)
                },
            };

            render_sticker_raw(&sticker, theme_id, Some(opts))
        })
        .collect::<Vec<Dom>>()
}

pub fn render_sticker_raw(sticker:&RawSticker, theme_id: ThemeId, opts: Option<StickerRawRenderOptions>) -> Dom {
    match sticker {
        RawSticker::Sprite(sprite) => render_sticker_sprite_raw(sprite, opts.map(|opts| opts.into_sprite_unchecked())),
        RawSticker::Text(text) => render_sticker_text_raw(text, theme_id, opts.map(|opts| opts.into_text_unchecked())),
    }
}

