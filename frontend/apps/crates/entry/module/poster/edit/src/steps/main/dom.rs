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
use super::stickers::dom::StickersDom;

pub struct MainDom {}
impl MainDom {
    pub fn render(state:Rc<State>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(StickersDom::render(state))
        })
    }
}
