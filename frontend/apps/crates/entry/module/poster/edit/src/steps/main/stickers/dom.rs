use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
use shared::domain::jig::module::body::{Sprite, Transform};
use super::state::*;
use components::transform::{
    dom::TransformDom,
};

pub struct StickersDom {}
impl StickersDom {
    pub fn render(state:Rc<State>) -> Dom {
        html!("empty-fragment", {
            .children_signal_vec(
                state.stickers.list.signal_vec_cloned().map(clone!(state => move |sticker| {
                    StickerDom::render(state.clone(), sticker)
                }))
            )
        })
    }
}

pub struct StickerDom {}
impl StickerDom {
    pub fn render(state:Rc<State>, sticker: Rc<Sticker>) -> Dom {
        //sticker.transform.lock_mut().scale.0 = [0.5, 0.5, 0.5];
        html!("empty-fragment", {
            .child(
                html!("img-ji", {
                    .visible_signal(sticker.loaded_signal())
                    .style_signal("width", sticker.width_signal())
                    .style_signal("height", sticker.height_signal())
                    .style_signal("transform", sticker.transform.matrix_string_signal())
                    .style("display", "block")
                    .style("position", "absolute")
                    .style("top", "0")
                    .style("left", "0")
                    .property("id", sticker.id.0.to_string())
                    .property("lib", sticker.lib.to_str())
                    .property("size", "full")
                    .event(clone!(sticker => move |evt:events::ImageLoad| {
                        sticker.transform.size.set(Some(evt.size()));
                    }))
                })
            )
            .child(TransformDom::render(sticker.transform.clone()))
        })
    }
}
