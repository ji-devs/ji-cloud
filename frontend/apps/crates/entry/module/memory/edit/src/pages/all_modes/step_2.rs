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
use crate::pages::all_modes::steps_nav::apply_steps_nav;

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
        _self.state.pairs
            .signal_vec_cloned()
            .enumerate()
            .map(clone!(_self => move |(index, (card_1, card_2))| {
                CardPairDom::render(CardPairDom::new(_self.state.clone(), index, card_1, card_2))
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

    pub fn render(_self: Rc<Self>, mode:GameMode) -> Dom {
        let el = match mode {
            GameMode::Duplicate => templates::duplicate::step_2_page(),
            GameMode::WordsAndImages => templates::words_and_images::step_2_page(),
        };
        elem!(el, { 
            .apply(|dom| apply_steps_nav(dom, _self.state.clone()))
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


pub struct CardPairDom {
    pub state: Rc<BaseGameState>,
    pub index: ReadOnlyMutable<Option<usize>>,
    pub is_hover:Mutable<Option<Side>>,
    pub card_1: Card,
    pub card_2: Card,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Left,
    Right
}

impl CardPairDom {
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
                                    .property_signal("src", src.signal_cloned())
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
        elem!(templates::card_pair_text_text_preview(), {
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





