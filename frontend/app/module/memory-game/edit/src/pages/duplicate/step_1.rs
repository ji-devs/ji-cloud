use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::utils::lines;
use std::fmt::Write;

pub struct Step1Page {
    pub words:MutableVec<String>,
    pub editing_text_content: Mutable<Option<String>>
}

impl Step1Page {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            words: MutableVec::new_with_values(vec!["hello".to_string(), "world".to_string()]), //MutableVec::new(),
            editing_text_content: Mutable::new(None)
        });

        _self
    }
   
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
}
