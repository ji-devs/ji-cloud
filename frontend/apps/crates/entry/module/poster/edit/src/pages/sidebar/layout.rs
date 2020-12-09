use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::config::LAYOUT_OPTIONS;

pub struct LayoutDom {
    pub state: Rc<State>,
}

impl LayoutDom {

    pub fn new(state:Rc<State>) -> Rc<Self> {
        Rc::new(Self { state })
    }

    pub fn render(_self:Rc<Self>) -> Dom {
        let state = _self.state.clone();

        elem!(templates::sidebar_layout(), {
            .with_data_id!("items", {
                .children(LAYOUT_OPTIONS
                    .iter()
                    .map(|layout| {
                        let id = layout.id;
                        elem!(templates::sidebar_layout_item(layout.label, &layout.thumbnail_url()), {
                            .event(move |evt:events::Click| {
                                //Each layout should have a raw::Poster associated, just reloads that
                                log::info!("todo - change layout to {}", id);
                            })
                        })
                    })
                    .collect::<Vec<Dom>>()
                )
            })
        })
    }
}
