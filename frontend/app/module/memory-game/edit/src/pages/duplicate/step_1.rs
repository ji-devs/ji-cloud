use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use std::fmt::Write;
use crate::data::*;
use itertools::Itertools;
pub struct Step1Page {
    state: Rc<DuplicateState>,
    
}

impl Step1Page {
    pub fn new(state:Rc<DuplicateState>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            state
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
                elem!(templates::card(), {
                    .with_data_id!("number", {
                        .text_signal(index.signal().map(|index| {
                            format!("{}", index.unwrap_or(0)+1)
                        }))
                    })
                    .with_data_id!("card-1", {
                        .with_data_id!("label" => HtmlTextAreaElement, {
                            .with_node!(elem => {
                                .event(clone!(_self,card => move |evt:events::Input| {
                                    let text = elem.value();
                                    card.text.set_neq(text);
                                }))
                            })
                            .property_signal("value", card.text.signal_cloned())
                        })
                    })
                    .with_data_id!("card-2", {
                        .with_data_id!("label" => HtmlTextAreaElement, {
                            .with_node!(elem => {
                                .event(clone!(_self,card => move |evt:events::Input| {
                                    let text = elem.value();
                                    card.text.set_neq(text);
                                }))
                            })
                            .property_signal("value", card.text.signal_cloned())
                        })
                    })
                })
            }))
    }

    fn text_input_signal(&self) -> impl Signal<Item = String> {
        self.state.cards
            .signal_vec_cloned()
            .map_signal(|card| card.text.signal_cloned())
            .to_signal_map(|texts| {
                texts.iter().join("\n")
            })
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::duplicate::step_1_page(), { 
            .with_data_id!("list-items" => HtmlTextAreaElement, {
                .with_node!(elem => {
                    .event(clone!(_self => move |evt:events::Input| {
                        let text = elem.value();
                        let mut cards:Vec<Card> = text.lines()
                            .map(|word| word.to_string().into())
                            .collect();

                        if(text.ends_with('\n')) {
                            cards.push("".to_string().into());
                        }
                        _self.state.cards.lock_mut().replace_cloned(cards);
                    }))
                    .property_signal("value", _self.text_input_signal())
                })
            })

            .with_data_id!("cards", {
                .children_signal_vec(Self::cards_dom_signal(_self))
            })
        })
    }
    /*
    pub fn editing_text_signal(&self) -> impl Signal<Item = bool> {
        self.editing_text_content.signal_ref(|x| x.is_some())
    }

    pub fn words_signal(&self) -> impl Signal<Item = String> {
        self.words
            .signal_vec_cloned()
            .to_signal_map(|words| {
                let mut blob = String::new();
                for word in words.iter() {
                    writeln!(&mut blob, "{}", word).unwrap_throw();
                }
                blob
            })
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::duplicate::step_1_page(), { 
            .with_data_id!("list-items", {
                .event(clone!(_self => move |evt:events::DoubleClick| {
                    if _self.editing_text_content.lock_ref().is_none() {
                        let words = _self.words.lock_ref();     
                        let mut blob = String::new();
                        for word in words.iter() {
                            writeln!(&mut blob, "{}", word).unwrap_throw();
                        }
                        _self.editing_text_content.set(Some(blob));
                    }
                }))
            })
            .with_data_id!("list-items-data", {
                .visible_signal(_self.editing_text_signal().map(|x| !x)) 
                .text_signal(_self.words_signal())
            })
            .with_data_id!("list-items-input" => HtmlTextAreaElement, {
                .visible_signal(_self.editing_text_signal())
                .focused_signal(_self.editing_text_signal())
                .with_node!(elem => {
                    .event(clone!(_self => move |evt:events::Input| {
                        let text = elem.value();
                        let words = lines::split_to_words(text);
                        _self.words.lock_mut().replace_cloned(words);
                    }))
                    .event(clone!(_self => move |evt:events::Blur| {
                        _self.editing_text_content.set(None);
                    }))
                    .property_signal("value", _self.editing_text_content.signal_ref(|x| {
                        match x {
                            None => "".to_string(),
                            Some(text) => text.to_string(),
                        }
                    }))
                })
            })
        })
    }
    */
}


