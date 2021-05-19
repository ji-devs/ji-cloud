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
    Stickers,Sticker,
    sprite::dom::SpriteDom,
    text::dom::{DebugOptions as TextDebugOptions, TextDom}
};

#[derive(Clone, Debug, Default)]
pub struct DebugOptions {
    pub text: Option<TextDebugOptions>, 
}
pub struct StickersDom {}
impl StickersDom {
    pub fn render(stickers:Rc<Stickers>, debug_opts: Option<DebugOptions>) -> Dom {
        let debug_opts = debug_opts.unwrap_or_default();

        html!("empty-fragment", {
            .children_signal_vec(
                stickers.list
                .signal_vec_cloned()
                .enumerate()
                .map(clone!(stickers => move |(index, sticker)| {
                    match sticker {
                        Sticker::Sprite(sprite) => SpriteDom::render(stickers.clone(), index, sprite),
                        Sticker::Text(text) => TextDom::render(stickers.clone(), index, text, debug_opts.text.clone()),
                    }
                }))
            )
        })
    }
}
