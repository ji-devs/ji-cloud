use dominator::{html, clone, Dom};
use crate::data::state::*;
use std::rc::Rc;
use futures_signals::{
    signal::SignalExt,
    signal_vec::SignalVecExt
};
use utils::prelude::*;

pub struct PlayerDom { }

impl PlayerDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(state.instructions.render(&state.audio_ctx))
            .child(html!("div", {.text("main")}))
        })
    }
}
