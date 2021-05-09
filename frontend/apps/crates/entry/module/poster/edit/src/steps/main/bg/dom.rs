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

pub struct BgDom {}
impl BgDom {
    pub fn render(state:Rc<State>) -> Dom {
        html!("empty-fragment", {
            .child_signal(state.bg.signal_cloned().map(|bg| {
                None
            }))
        })
    }
}
