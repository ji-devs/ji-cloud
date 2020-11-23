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
use crate::pages::all_modes::steps_nav::apply_steps_nav;
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
 
    fn cards_dom_signal(_self: Rc<Self>) -> impl SignalVec<Item = Dom> {
        _self.state.edit_text_list
            .signal_vec_cloned()
            //this allows us to hide the visuals of empty cards, but it gets weird
            //.filter_signal_cloned(|card| card.text.signal_ref(|text| !text.is_empty()))
            .enumerate()
            .map(clone!(_self => move |(index, text)| {
                elem!(templates::card_pair_text_text_edit(), {
                    .with_data_id!("number", {
                        .text_signal(index.signal().map(|index| {
                            format!("{}", index.unwrap_or(0)+1)
                        }))
                    })
                    .with_data_id!("left", {
                        .with_data_id!("text-contents" => HtmlTextAreaElement, {
                            .with_node!(elem => {
                                .event(clone!(_self, index => move |evt:events::Input| {
                                    let value = elem.value();
                                    let index = index.get().unwrap_or(0);

                                    _self.state.edit_text_list
                                        .lock_ref()
                                        .get(index)
                                        .unwrap_throw()
                                        .set(value);
                                }))
                            })
                            .property_signal("value", text.signal_cloned())
                        })
                    })
                    .with_data_id!("right", {
                        .with_data_id!("text-contents" => HtmlTextAreaElement, {
                            .with_node!(elem => {
                                .event(clone!(_self, index => move |evt:events::Input| {
                                    let value = elem.value();
                                    let index = index.get().unwrap_or(0);

                                    _self.state.edit_text_list
                                        .lock_ref()
                                        .get(index)
                                        .unwrap_throw()
                                        .set(value);
                                }))
                            })
                            .property_signal("value", text.signal_cloned())
                        })
                    })
                })
            }))
    }

    fn words_signal(&self) -> impl Signal<Item = Vec<String>> {
        self.state.edit_text_list
            .signal_vec_cloned()
            .map_signal(|x| x.signal_cloned())
            .to_signal_map(|x| x.iter().map(|x| x.to_string()).collect())
    }
    fn text_input_signal(&self) -> impl Signal<Item = String> {
        self.state.edit_text_list
            .signal_vec_cloned()
            .map_signal(|x| x.signal_cloned())
            .to_signal_map(|x| x.join("\n"))
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::duplicate::step_1_page(), { 
            .future(_self.words_signal().for_each(clone!(_self => move |words| {
                _self.state.pairs
                    .lock_mut()
                    .replace_cloned(
                        words
                            .into_iter()
                            .map(|word|  (word.clone().into(), word.into()))
                            .collect()
                    );
                async {}
            })))

            .apply(|dom| apply_steps_nav(dom, _self.state.clone()))

            .with_data_id!("list-items" => HtmlTextAreaElement, {
                .with_node!(elem => {
                    .event(clone!(_self => move |evt:events::Input| {
                        let text = elem.value();
                        let mut words:Vec<Mutable<String>> = text.lines()
                            .map(|word| Mutable::new(word.to_string()))
                            .collect();

                        if(text.ends_with('\n')) {
                            words.push(Mutable::new("".to_string().into()));
                        }

                        _self.state.edit_text_list.lock_mut().replace_cloned(words);
                    }))
                    .property_signal("value", _self.text_input_signal())
                })
            })

            .with_data_id!("cards", {
                .dynamic_class_signal!(_self.state.theme_id.signal_ref(|id| {
                    Some(format!("memory-theme-{}", id))
                }))
                .children_signal_vec(Self::cards_dom_signal(_self.clone()))
            })
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


