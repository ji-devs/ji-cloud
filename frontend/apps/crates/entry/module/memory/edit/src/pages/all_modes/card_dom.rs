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
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::config;
use crate::templates;

pub fn apply_edit_cards(dom:DomBuilder<HtmlElement>, state: Rc<BaseGameState>) -> DomBuilder<HtmlElement> {
    apply_methods!(dom, {
        .with_data_id!("cards", {
            .dynamic_class_signal!(state.theme_id.signal_ref(|id| {
                Some(format!("memory-theme-{}", id))
            }))
            .children_signal_vec(BaseGameState::cards_edit_dom_signal(state.clone()))
        })
    })
}

pub fn apply_preview_cards(dom:DomBuilder<HtmlElement>, state: Rc<BaseGameState>) -> DomBuilder<HtmlElement> {
    apply_methods!(dom, {
        .with_data_id!("cards", {
            .dynamic_class_signal!(state.theme_id.signal_ref(|id| {
                Some(format!("memory-theme-{}", id))
            }))
            .children_signal_vec(BaseGameState::cards_preview_dom_signal(state.clone()))
        })
    })
}
pub struct CardPairEditDom {
    pub state: Rc<BaseGameState>,
    pub index: ReadOnlyMutable<Option<usize>>,
    pub card_1: Card,
    pub card_2: Card,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Left,
    Right
}

impl CardPairEditDom {
    pub fn new(state:Rc<BaseGameState>, index:ReadOnlyMutable<Option<usize>>, card_1: Card, card_2: Card) -> Rc<Self> {
        Rc::new(Self {
            state,
            index,
            card_1,
            card_2
        })
    }

    fn render_side(_self: Rc<Self>, dom:DomBuilder<HtmlElement>, mode: GameMode, side:Side) -> DomBuilder<HtmlElement> {
        let side_name:&'static str = if side == Side::Left { "left" } else { "right" };
        let card = if side == Side::Left { &_self.card_1 } else { &_self.card_2 };

        apply_methods!(dom, {
            .with_data_id!(side_name, {
                .apply(|dom| {
                    match card {
                        Card::Text(text) => {
                            apply_methods!(dom, {
                                .with_data_id!("text-contents" => HtmlTextAreaElement, {
                                    .text_signal(text.signal_cloned())
                                    .with_node!(elem => {
                                        .event(clone!(_self, text => move |evt:events::Input| {
                                            let value = elem.value();
                                            let index = _self.index.get().unwrap_or(0);

                                            _self.state.edit_text_list
                                                .lock_mut()
                                                .set_cloned(index, value.clone());
                                            
                                            text.set_neq(value.clone()); 


                                            match mode {
                                                GameMode::Duplicate => {

                                                    let other_card = if side == Side::Left { 
                                                        &_self.card_2 
                                                    } else { 
                                                        &_self.card_1 
                                                    };
                                                    match other_card {
                                                        Card::Text(other_text) => {
                                                            other_text.set_neq(value);
                                                        },
                                                        _ => {}
                                                    }
                                                },
                                                _ => {}
                                            };
                                        }))
                                    })
                                })
                            })
                        },
                        Card::Image(src) => {
                            apply_methods!(dom, {
                                .with_data_id!("image", {
                                    .class_signal("hidden", src.signal_ref(|x| x.is_none()))
                                    .property_signal("src", src.signal_cloned())
                                })
                                .with_data_id!("image-waiting", {
                                    .class_signal("hidden", src.signal_ref(|x| x.is_some()))
                                })
                            })
                        },
                        _ => unimplemented!("don't know how to render audio!")
                    }
                })
            })
        })
    }
    pub fn render(_self: Rc<Self>) -> Dom {
        let mode = _self.state.mode;
        let el = match (&_self.card_1, &_self.card_2) {
            (Card::Text(_), Card::Text(_)) => templates::card_pair_text_text_edit(),
            (Card::Text(_), Card::Image(_)) => templates::card_pair_text_image_edit(),
            _ => unimplemented!("don't know how to render this kind of card pair!")
        };
        elem!(el, {
            .with_data_id!("number", {
                .text_signal(_self.index.signal().map(|index| {
                    format!("{}", index.unwrap_or(0)+1)
                }))
            })
            .apply(|dom| Self::render_side(_self.clone(), dom, mode, Side::Left))
            .apply(|dom| Self::render_side(_self.clone(), dom, mode, Side::Right))
        })
    }
}


pub struct CardPairPreviewDom {
    pub state: Rc<BaseGameState>,
    pub index: ReadOnlyMutable<Option<usize>>,
    pub is_hover:Mutable<Option<Side>>,
    pub card_1: Card,
    pub card_2: Card,
}

impl CardPairPreviewDom {
    pub fn new(state:Rc<BaseGameState>, index:ReadOnlyMutable<Option<usize>>, card_1: Card, card_2: Card) -> Rc<Self> {
        Rc::new(Self {
            state,
            index,
            is_hover: Mutable::new(None),
            card_1,
            card_2
        })
    }

    fn render_side(_self: Rc<Self>, dom:DomBuilder<HtmlElement>, side:Side) -> DomBuilder<HtmlElement> {
        let side_name:&'static str = if side == Side::Left { "left" } else { "right" };
        let card = if side == Side::Left { &_self.card_1 } else { &_self.card_2 };

        apply_methods!(dom, {
            .with_data_id!(side_name, {
                .class_signal("flip-card-clicked", _self.is_hover.signal().map(move |hover| hover == Some(side)))
                .with_node!(element => {
                    .event(clone!(_self => move |evt:events::MouseEnter| {
                        _self.is_hover.set(Some(side));
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
                .apply(|dom| {
                    match card {
                        Card::Text(text) => {
                            apply_methods!(dom, {
                                .with_data_id!("text-contents", {
                                    .text_signal(text.signal_cloned())
                                })
                            })
                        },
                        Card::Image(src) => {

                            apply_methods!(dom, {
                                .with_data_id!("image", {
                                    .class_signal("hidden", src.signal_ref(|x| x.is_none()))
                                    .property_signal("src", src.signal_cloned())
                                })
                                .with_data_id!("image-waiting", {
                                    .class_signal("hidden", src.signal_ref(|x| x.is_some()))
                                })
                            })
                        },
                        _ => unimplemented!("don't know how to render audio!")
                    }
                })
            })
        })
    }
    pub fn render(_self: Rc<Self>) -> Dom { 
        let el = match (&_self.card_1, &_self.card_2) {
            (Card::Text(_), Card::Text(_)) => templates::card_pair_text_text_preview(),
            (Card::Text(_), Card::Image(_)) => templates::card_pair_text_image_preview(),
            _ => unimplemented!("don't know how to render this kind of card pair!")
        };
        elem!(el, {
            .with_data_id!("number", {
                .text_signal(_self.index.signal().map(|index| {
                    format!("{}", index.unwrap_or(0)+1)
                }))
            })
            .apply(|dom| Self::render_side(_self.clone(), dom, Side::Left))
            .apply(|dom| Self::render_side(_self.clone(), dom, Side::Right))
        })
    }
}
