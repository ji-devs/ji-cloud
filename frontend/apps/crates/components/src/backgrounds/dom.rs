use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
use super::state::*;

#[derive(Clone, Debug, Default)]
pub struct DebugOptions {
}

pub fn render(bg:Rc<Backgrounds>, debug_opts: Option<DebugOptions>) -> Dom {
    let debug_opts = debug_opts.unwrap_or_default();

    html!("empty-fragment", {
    })
}
