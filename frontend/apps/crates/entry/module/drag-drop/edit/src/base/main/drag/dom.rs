use components::module::_common::edit::prelude::*;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::state::*;
use components::{
    backgrounds::dom::render_backgrounds, 
    stickers::dom::{render_stickers, render_stickers_raw},
    traces::edit::dom::render_traces_edit
};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

impl MainDrag {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div")
    }
}
 
