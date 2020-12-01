use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::config::LAYOUT_OPTIONS;
use utils::components::image::transform::TransformImage;

pub struct MainDom {
    pub state: Rc<State>,
}

impl MainDom {

    pub fn new(state:Rc<State>) -> Rc<Self> {
        Rc::new(Self { state })
    }

    fn children_signal(_self: Rc<Self>) -> impl SignalVec<Item = Dom> {
        _self.state.poster.images.signal_vec_cloned()
            .map(|img| {
                TransformImage::render(img)
            })
    }
    pub fn render(_self:Rc<Self>) -> Dom {
        let state = _self.state.clone();

        elem!(templates::main(), {
            .children_signal_vec(Self::children_signal(_self))
        })
    }
}
