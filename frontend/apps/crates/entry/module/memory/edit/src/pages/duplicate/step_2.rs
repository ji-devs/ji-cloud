use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, spawn_future, dynamic_class_signal, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use std::fmt::Write;
use crate::data::*;
use itertools::Itertools;
use crate::config;

pub struct ThemeOption {
    pub state: Rc<BaseGameState>,
    pub is_hover:Mutable<bool>,
    pub theme: Theme,
}

impl ThemeOption {
    pub fn new(state:Rc<BaseGameState>, theme: Theme) -> Rc<Self> {
        Rc::new(Self {
            state,
            is_hover: Mutable::new(false),
            theme
        })
    }

    fn text_signal(&self) -> impl Signal<Item = &'static str> {
        self.is_hover
            .signal()
            .map(|hover| {
                if hover {
                    config::THEME_EXAMPLE_TEXT_2
                } else {
                    config::THEME_EXAMPLE_TEXT_1
                }
            })
    }
    pub fn render(_self: Rc<Self>) -> Dom { 
        html!("div", {
            .child_signal(_self.state.theme_id
                .signal_cloned()
                .map(clone!(_self => move |theme_id| { 
                    let selected = _self.theme.id == theme_id;
                    Some(
                    
                        elem!(templates::step_2_theme_item(selected), {

                            .with_node!(element => {
                                .class(&format!("memory-theme-{}", _self.theme.id))
                                .with_data_id!("left", {
                                    .with_data_id!("text-contents", {
                                        .text_signal(_self.text_signal())
                                    })
                                })
                                .with_data_id!("label", {
                                    .text(_self.theme.label)
                                })

                                .event(clone!(_self => move |evt:events::Click| {
                                    _self.state.theme_id.set(_self.theme.id.to_string());
                                }))
                                .event(clone!(_self => move |evt:events::MouseEnter| {
                                    _self.is_hover.set(true);
                                }))
                                .event_preventable(clone!(_self => move |evt:events::MouseLeave| {
                                    if let Some(target) = evt.target() {
                                        if target == element.clone().unchecked_into() {
                                            _self.is_hover.set(false);
                                        } else {
                                            evt.prevent_default();
                                        }
                                    }
                                }))
                            })
                        })
                    )
                }))
            )
        })
    }
}


pub struct CardDom {
    pub state: Rc<BaseGameState>,
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
    pub fn new(state:Rc<BaseGameState>, index:ReadOnlyMutable<Option<usize>>, card: Card) -> Rc<Self> {
        Rc::new(Self {
            state,
            index,
            is_hover: Mutable::new(None),
            card 
        })
    }
    pub fn render(_self: Rc<Self>) -> Dom { 
        elem!(templates::card_pair_text_text_preview(), {
            .with_data_id!("number", {
                .text_signal(_self.index.signal().map(|index| {
                    format!("{}", index.unwrap_or(0)+1)
                }))
            })
            //TODO - macro to make hover stuff DRY
            .with_data_id!("left", {

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
            .with_data_id!("right", {
                .class_signal("flip-card-clicked", _self.is_hover.signal().map(|hover| hover == Some(Side::Right)))
                .with_node!(element => {
                    .event(clone!(_self => move |evt:events::MouseEnter| {
                        _self.is_hover.set(Some(Side::Right));
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
        })
    }
}

pub struct Step2Page {
    state: Rc<BaseGameState>,
}

impl Step2Page {
    pub fn new(state:Rc<BaseGameState>) -> Rc<Self> {

        let preview_theme_id = Mutable::new(state.theme_id.get_cloned());
        let _self = Rc::new(Self { 
            state,
        });

        _self
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

    fn theme_options_dom(_self: Rc<Self>) -> impl Iterator<Item = Dom> {
        config::THEME_OPTIONS
            .iter()
            .map(clone!(_self => move |theme| {
                ThemeOption::render(ThemeOption::new(
                    _self.state.clone(), 
                    theme.clone()
                ))
            }))
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::duplicate::step_2_page(), { 
            .with_data_id!("top-step-1", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.state.step.set(Step::One);
                }))
            })
            .with_data_id!("cards", {
                .dynamic_class_signal!(_self.state.theme_id.signal_ref(|id| {
                    Some(format!("memory-theme-{}", id))
                }))
                .children_signal_vec(Self::cards_dom_signal(_self.clone()))
            })
            .with_data_id!("theme-items", {
                .children(Self::theme_options_dom(_self.clone()))
            })
            .with_data_id!("next", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.state.step.set(Step::Four);
                }))
            })
        })
    }
}


