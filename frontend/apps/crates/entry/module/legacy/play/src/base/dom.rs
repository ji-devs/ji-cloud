use components::{
    module::_common::play::prelude::DomRenderable,
    backgrounds::dom::render_backgrounds_raw,
    stickers::dom::render_stickers_raw
};
use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::{
    state::*,
    design::*
};
use components::{backgrounds, stickers, traces};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .children(&mut [
                state.render_design()
            ])
        })
    }
}
