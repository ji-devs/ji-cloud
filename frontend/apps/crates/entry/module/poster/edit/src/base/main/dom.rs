use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::state::*;
use components::{
    module::_common::edit::prelude::*,
    backgrounds::dom::render_backgrounds, 
    stickers::dom::{render_stickers, render_stickers_raw},
};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment", {
            .child(render_stickers(state.base.stickers.clone()))
        })
    }
}
impl MainDomRenderable for Main {
    fn render_bg(state: Rc<Main>) -> Option<Dom> {
        Some(render_backgrounds(state.base.backgrounds.clone(), None))
    }
}
