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
    sprite,
    text,
};
use shared::domain::jig::module::body::_groups::design::Sticker as RawSticker;

#[derive(Clone, Debug, Default)]
pub struct DebugOptions {
    pub text: Option<text::dom::DebugOptions>, 
}

pub fn render(stickers:Rc<Stickers>, debug_opts: Option<DebugOptions>) -> Dom {
    let debug_opts = debug_opts.unwrap_or_default();

    html!("empty-fragment", {
        .children_signal_vec(
            stickers.list
            .signal_vec_cloned()
            .enumerate()
            .map(clone!(stickers => move |(index, sticker)| {
                match sticker {
                    Sticker::Sprite(sprite) => sprite::dom::render(stickers.clone(), index, sprite),
                    Sticker::Text(text) => text::dom::render(stickers.clone(), index, text, debug_opts.text.clone()),
                }
            }))
        )
    })
}
pub fn render_raw(stickers:&[RawSticker]) -> Dom {
    html!("empty-fragment", {
        .children(
            stickers
                .iter()
                .map(|sticker| {
                    match sticker {
                        RawSticker::Sprite(sprite) => sprite::dom::render_raw(sprite),
                        RawSticker::Text(text) => text::dom::render_raw(text),
                    }
                })
                .collect::<Vec<Dom>>()
        )
    })
}
