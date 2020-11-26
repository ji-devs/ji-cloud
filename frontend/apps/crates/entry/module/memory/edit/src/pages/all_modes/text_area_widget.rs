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

pub fn apply_text_area_widget(dom:DomBuilder<HtmlElement>, state: Rc<BaseGameState>) -> DomBuilder<HtmlElement> {
    apply_methods!(dom, {
        .with_data_id!("list-items" => HtmlTextAreaElement, {
            .with_node!(elem => {
                .event(clone!(state => move |evt:events::Input| {
                    let text = elem.value();
                    let mut words:Vec<String> = text.lines()
                        .map(|word| word.to_string()) 
                        .collect();

                    if(text.ends_with('\n')) {
                        words.push("".to_string().into());
                    }
                    state.edit_text_list.lock_mut().replace_cloned(words.clone());

                    // TODO - THIS WILL DELETE AND RE-CREATE ALL CARD DATA
                    // IT'S HORRIBLE
                    // BUT THERE IS NOTHING WE CAN DO FOR NOW
                    state.pairs
                        .lock_mut()
                        .replace_cloned(
                            words
                                .iter()
                                .map(|word| {
                                    match state.mode {
                                        GameMode::Duplicate => {
                                            (
                                                Card::new_text(word.to_string()),
                                                Card::new_text(word.to_string()),
                                            )
                                        },
                                        GameMode::WordsAndImages => {
                                            (
                                                Card::new_text(word.to_string()),
                                                Card::new_image(None),
                                            )
                                        },
                                    }
                                })
                                .collect()
                        );
                }))
                .property_signal("value", state.text_input_signal())
            })
        })
    })
}
