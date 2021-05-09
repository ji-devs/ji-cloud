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
use crate::steps::main::{
    stickers::dom::StickerDom,
    text::dom::TextDom
};

pub struct RenderablesDom {}
impl RenderablesDom {
    pub fn render(state:Rc<State>) -> Dom {
        html!("empty-fragment", {
            .children_signal_vec(
                state.renderables.list
                .signal_vec_cloned()
                .enumerate()
                .map(clone!(state => move |(index, renderable)| {
                    match renderable {
                        Renderable::Sticker(sticker) => StickerDom::render(state.clone(), index, sticker),
                        Renderable::Text(text) => TextDom::render(state.clone(), index, text),
                    }
                }))
            )
        })
    }
}
