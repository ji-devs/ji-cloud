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

use dominator_helpers::{elem, with_data_id};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::debug;
use super::steps_nav::apply_steps_nav;


pub struct Header {
    state: Rc<State>, 
    game_mode: GameMode,
}

#[derive(Clone, PartialEq, Debug)]
enum HeaderMode {
    AddCards,
    Empty,
    Preview
}
impl Header {

    pub fn render(state: Rc<State>, game_mode:GameMode) -> impl Signal<Item = Dom> {
        let _self = Self::new(state, game_mode);

        _self.header_mode_signal().map(clone!(_self => move |header_mode| {
            match header_mode {
                HeaderMode::AddCards => Self::render_add(_self.clone()),
                HeaderMode::Preview => Self::render_preview(_self.clone()),
                HeaderMode::Empty => Self::render_empty(_self.clone()),
            }
        }))
    }

    fn new(state: Rc<State>, game_mode:GameMode) -> Rc<Self> {
        Rc::new(Self { 
            state, 
            game_mode,
        })
    }

    fn header_mode_signal(&self) -> impl Signal<Item = HeaderMode> {
        map_ref! {
            let step = self.state.step.signal(),
            let cards_edit = self.state.cards_edit_signal()
            => {
                let step = *step;
                let cards_edit = *cards_edit;
                
                if step == Step::Four {
                    HeaderMode::Preview
                } else if cards_edit {
                    HeaderMode::AddCards
                } else {
                    HeaderMode::Empty
                }
            }
        }
    }
    fn render_preview(_self: Rc<Self>) -> Dom {
        let state = _self.state.clone();
        elem!(templates::header_preview(), {
            .apply(|dom| apply_steps_nav(dom, _self.state.clone()))
        })
    }
    fn render_empty(_self: Rc<Self>) -> Dom {
        let state = _self.state.clone();
        elem!(templates::header_empty(), {
            .with_data_id!("btn-preview", {
                .event(clone!(state => move |evt:events::Click| {
                    state.step.set(Step::Four);
                }))
            })
        })
    }
    fn render_add(_self: Rc<Self>) -> Dom {
        let state = _self.state.clone();
        elem!(templates::header_add_pair(), {
            .with_data_id!("btn-preview", {
                .event(clone!(state => move |evt:events::Click| {
                    state.step.set(Step::Four);
                }))
            })
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
