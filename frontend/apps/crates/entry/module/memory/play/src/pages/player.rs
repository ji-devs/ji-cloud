use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable,ReadOnlyMutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{Url, HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem,dynamic_class_signal ,with_data_id, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use utils::{
    iframe::*,
    components::module_page::*,
};
use crate::{debug, data::{*, raw::*}};
use std::future::Future;
use async_trait::async_trait;
use std::{
    pin,
    future,
    marker
};

pub struct PlayerPage {
    state: GameState,
}

impl PlayerPage {
    pub fn new(jig_id: String, module_id: String) -> Rc<Self> {
        let state = GameState::new(jig_id, module_id);
        Rc::new(Self { state })
    }
}

//Use the ModuleRenderer component by way of a trait
#[async_trait(?Send)]
impl ModuleRenderer for PlayerPage {
    type Data = GameStateRaw;

    async fn load(_self:Rc<Self>) -> GameStateRaw { 
        if let Some(raw_state) = debug::settings().state {
            raw_state
        } else {
            log::info!("loading...");
            GameStateRaw::load().await
        }
    }

    fn render(_self: Rc<Self>, data: GameStateRaw) -> Dom {
        _self.state.set_from_loaded(data);
        html!("div", {
            .child_signal(_self.state.mode.signal_ref(clone!(_self => move |mode| {
                mode.map(clone!(_self => move |_| {
                    match _self.state.mode_state.borrow().as_ref() {
                        None => panic!("can't render player without state!"),
                        Some(mode) => match mode {
                            ModeState::Duplicate(state) => {
                                DuplicatePlayer::render(DuplicatePlayer::new(state.clone()))
                            },
                            _ => unimplemented!("todo - other modes!")
                        }
                    }
                }))
            })))
        })
    }
}


pub struct DuplicatePlayer {
    state: Rc<DuplicateState>,
}

impl DuplicatePlayer {
    pub fn new(state:Rc<DuplicateState>) -> Rc<Self> {
        Rc::new(Self { state })
    }

    pub fn render(_self:Rc<DuplicatePlayer>) -> Dom {
        elem!(templates::player(), { 
            .with_data_id!("cards", {
                .dynamic_class_signal!(_self.state.theme_id.signal_ref(|id| {
                    Some(format!("memory-theme-{}", id))
                }))
                .children_signal_vec(Self::cards_dom_signal(_self.clone()))
            })
        })
    }

    fn cards_dom_signal(_self: Rc<Self>) -> impl SignalVec<Item = Dom> {
        _self.state.cards
            .signal_vec_cloned()
            //this allows us to hide the visuals of empty cards, but it gets weird
            //.filter_signal_cloned(|card| card.text.signal_ref(|text| !text.is_empty()))
            .enumerate()
            .map(clone!(_self => move |(index, card)| {
                CardDom::render(CardDom::new(_self.state.clone(), index, card))
            }))
    }
}

pub struct CardDom {
    pub state: Rc<DuplicateState>,
    pub index: ReadOnlyMutable<Option<usize>>,
    pub is_hover:Mutable<Option<Side>>,
    pub card: Card,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Left,
    Right
}

impl CardDom {
    pub fn new(state:Rc<DuplicateState>, index:ReadOnlyMutable<Option<usize>>, card: Card) -> Rc<Self> {
        Rc::new(Self {
            state,
            index,
            is_hover: Mutable::new(None),
            card 
        })
    }
    pub fn render(_self: Rc<Self>) -> Dom { 
        elem!(templates::card(), {

            .class_signal("flip-card-clicked", _self.is_hover.signal().map(|hover| hover == Some(Side::Left)))
            .with_node!(element => {
                .event(clone!(_self => move |evt:events::MouseEnter| {
                    _self.is_hover.set(Some(Side::Left));
                }))
                .event_preventable(clone!(_self => move |evt:events::MouseLeave| {
                    if let Some(target) = evt.target() {
                        if target == element.clone().unchecked_into() {
                            _self.is_hover.set(None);
                        } else {
                            evt.prevent_default();
                        }
                    }
                }))
            })
            .with_data_id!("text-contents", {
                .text_signal(_self.card.text.signal_cloned())
            })
        })
    }
}
