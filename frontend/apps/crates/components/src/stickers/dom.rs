use dominator::{Dom, DomBuilder, clone, html};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{ReadOnlyMutable, Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use super::{
    state::*,
    sprite::dom::{
        render_sticker_sprite, 
        render_sticker_sprite_raw, 
        render_sticker_sprite_raw_mixin, 
        render_sticker_sprite_raw_parent_mixin,
        render_sticker_sprite_raw_offset, 
        render_sticker_sprite_raw_offset_mixin, 
        render_sticker_sprite_raw_offset_parent_mixin,
    }, 
    text::dom::{
        render_sticker_text, 
        render_sticker_text_raw, 
        render_sticker_text_raw_mixin, 
        render_sticker_text_raw_parent_mixin,
        render_sticker_text_raw_offset, 
        render_sticker_text_raw_offset_mixin, 
        render_sticker_text_raw_offset_parent_mixin,
    }
};
use web_sys::HtmlElement;
use shared::domain::jig::module::body::_groups::design::Sticker as RawSticker;

pub type OffsetMutable = ReadOnlyMutable<(f64, f64)>;

/* TODO - not all these patterns are really used... clean up or make it more about providing
 * options?
 */

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
        .map(|sticker| {
            match sticker {
                RawSticker::Sprite(sprite) => render_sticker_sprite_raw(sprite),
                RawSticker::Text(text) => render_sticker_text_raw(text, theme_id),
            }
        })
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
            match sticker {
                RawSticker::Sprite(sprite) => render_sticker_sprite_raw_mixin(sprite, mixin.clone()),
                RawSticker::Text(text) => render_sticker_text_raw_mixin(text, theme_id, mixin.clone()),
            }
        })
        .collect::<Vec<Dom>>()
}

pub fn render_sticker_raw(sticker:&RawSticker, theme_id: ThemeId) -> Dom {
    match sticker {
        RawSticker::Sprite(sprite) => render_sticker_sprite_raw(sprite),
        RawSticker::Text(text) => render_sticker_text_raw(text, theme_id),
    }
}

pub fn render_sticker_raw_offset_mixin<F>(sticker:&RawSticker, theme_id: ThemeId, offset: OffsetMutable, mixin: F) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static
{
    match sticker {
        RawSticker::Sprite(sprite) => render_sticker_sprite_raw_offset_mixin(sprite, offset, mixin),
        RawSticker::Text(text) => render_sticker_text_raw_offset_mixin(text, theme_id, offset, mixin),
    }
}

pub fn render_sticker_raw_mixin<F>(sticker:&RawSticker, theme_id: ThemeId, mixin: F) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static
{
    match sticker {
        RawSticker::Sprite(sprite) => render_sticker_sprite_raw_mixin(sprite, mixin),
        RawSticker::Text(text) => render_sticker_text_raw_mixin(text, theme_id, mixin),
    }
}
//Yeah it's a bit weird, but helpful for creating generic containers like StickerOutline
//The idea is that the sticker sets styles on the parent and then appends itself
//So the parent gets transformed etc.
pub fn render_sticker_raw_parent_mixin<F>(sticker:&RawSticker, theme_id: ThemeId, parent: DomBuilder<HtmlElement>, child_mixin: F) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static
{
    match sticker {
        RawSticker::Sprite(sprite) => render_sticker_sprite_raw_parent_mixin(parent, sprite, child_mixin),
        RawSticker::Text(text) => render_sticker_text_raw_parent_mixin(parent, text, theme_id, child_mixin),
    }
}
