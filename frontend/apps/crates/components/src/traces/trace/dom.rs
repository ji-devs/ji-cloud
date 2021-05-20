use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{always, Signal, Mutable, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use super::{
    super::state::*,
    state::*,
};
use shared::domain::jig::module::body::Sticker as RawSticker;

pub fn render_edit(traces:Rc<Traces>, index: ReadOnlyMutable<Option<usize>>, trace: Rc<Trace>) -> Dom {

    html!("empty-fragment", {
    })
}
