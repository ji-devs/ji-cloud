use components::module::edit::prelude::*;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::state::*;
use components::{backgrounds, stickers, traces};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .children(&mut [
                backgrounds::dom::render(state.base.backgrounds.clone(), None),
                stickers::dom::render(state.base.stickers.clone(), None)
            ])
        })
    }
}
