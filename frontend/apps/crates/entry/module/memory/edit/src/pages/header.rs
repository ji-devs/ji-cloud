use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, ReadOnlyMutable},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::debug;
use utils::components::module_page::*;
use async_trait::async_trait;
use super::steps_nav::apply_steps_nav;


pub struct Header {
    state: Rc<State>, 
    game_mode: GameMode,
}

impl Header {
    pub fn new(state: Rc<State>, game_mode:GameMode) -> Rc<Self> {
        Rc::new(Self { 
            state, 
            game_mode,
        })
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(_self.state.cards_edit_signal().map(clone!(_self => move |header_add| Some(
                if header_add {
                    Self::render_add(_self.clone())
                } else {
                    elem!(templates::header_empty(), { })
                }
            ))))
        })
    }
    fn render_add(_self: Rc<Self>) -> Dom {
        let state = _self.state.clone();
        elem!(templates::header_add_pair(), {
            .with_data_id!("add-btn", {
                .event(clone!(state => move |evt:events::Click| {
                    let game_mode = state.game_mode.get_cloned();
                    let pair = match game_mode.unwrap_throw() {
                        GameMode::Duplicate => {
                            (Card::new(CardMode::Text), Card::new(CardMode::Text))
                        },
                        GameMode::WordsAndImages => {
                            (Card::new(CardMode::Text), Card::new(CardMode::Image))
                        },
                        _ => {
                            unimplemented!("don't know how to add this pair!")
                        }
                    };

                    state.pairs.lock_mut().push_cloned(pair);
                }))
            })
        })
    }
}
