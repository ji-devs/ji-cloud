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
use components::image::{
    data::*,
    search::*
};

pub struct ImagesDom {
    pub state: Rc<State>,
}

impl ImagesDom {

    pub fn new(state:Rc<State>) -> Rc<Self> {
        Rc::new(Self { state })
    }

    pub fn render(_self:Rc<Self>) -> Dom {
        let state = _self.state.clone();

        elem!(templates::sidebar_images(), {
            .with_data_id!("search-widget", {
                .child(ImageSearchWidget::render(
                    ImageSearchWidget::new(
                        crate::debug::settings().image_search,
                        Some(clone!(state => move |img:MetaImage| {
                            state.poster.add_image(img.into());
                        }))
                    )
                ))
            })
        })
    }
}
