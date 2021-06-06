use components::module::play::prelude::DomRenderable;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::state::*;
use components::{backgrounds, stickers, traces};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(html!("div", {
                .text(&state.text)
            }))
        })
    }
}
