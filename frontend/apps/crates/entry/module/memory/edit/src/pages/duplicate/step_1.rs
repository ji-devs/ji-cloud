use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{CancelableFutureHandle, map_ref, signal::{Mutable, MutableSignal, Signal, SignalExt}, signal_vec::{MutableVec, SignalVecExt, SignalVec}};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, dynamic_class_signal, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use std::fmt::Write;
use crate::data::*;
use crate::pages::all_modes::{
    steps_nav::apply_steps_nav,
    text_area_widget::apply_text_area_widget,
    card_dom::apply_edit_cards,
};
use itertools::Itertools;
pub struct Step1Page {
    state: Rc<BaseGameState>,
}

impl Step1Page {
    pub fn new(state:Rc<BaseGameState>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            state
        });

        _self
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::duplicate::step_1_page(), { 
            .apply(|dom| apply_steps_nav(dom, _self.state.clone()))
            .apply(|dom| apply_text_area_widget(dom, _self.state.clone()))
            .apply(|dom| apply_edit_cards(dom, _self.state.clone()))
            .with_data_id!("next", {
                .event(clone!(_self => move |evt:events::Click| {
                    /*
                    let len = _self.state.pairs.lock_ref().len();

                    if len < 3 {
                        //TODO - show error
                    }
                    */
                    _self.state.step.set(Step::Two);
                }))
            })
        })
    }
}
