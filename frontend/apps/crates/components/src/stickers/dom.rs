use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
use super::{
    state::*,
    sprite::dom::{render_sticker_sprite, render_sticker_sprite_raw},
    text::dom::{render_sticker_text, render_sticker_text_raw},
};
use shared::domain::jig::module::body::_groups::design::Sticker as RawSticker;

pub fn render_stickers(stickers:Rc<Stickers>) -> Dom {
    html!("empty-fragment", {
        .children_signal_vec(
            stickers.list
            .signal_vec_cloned()
            .enumerate()
            .map(clone!(stickers => move |(index, sticker)| {
                match sticker {
                    Sticker::Sprite(sprite) => render_sticker_sprite(stickers.clone(), index, sprite),
                    Sticker::Text(text) => render_sticker_text(stickers.clone(), index, text),
                }
            }))
        )
    })
}

pub fn render_stickers_raw(stickers:&[RawSticker]) -> Dom {
    html!("empty-fragment", {
        .children(
            stickers
                .iter()
                .map(|sticker| {
                    match sticker {
                        RawSticker::Sprite(sprite) => render_sticker_sprite_raw(sprite),
                        RawSticker::Text(text) => render_sticker_text_raw(text),
                    }
                })
                .collect::<Vec<Dom>>()
        )
    })
}
