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
use components::image::{
    data::*,
    search::*
};

use super::steps_nav::apply_steps_nav;

pub struct Step1Sidebar {
    state: Rc<State>, 
    game_mode: GameMode,
    text_area: RefCell<Option<HtmlTextAreaElement>>
}

impl Step1Sidebar {
    pub fn new(state: Rc<State>, game_mode:GameMode) -> Rc<Self> {
        Rc::new(Self { 
            state, 
            game_mode,
            text_area: RefCell::new(None)
        })
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", {
            .class("w-full")
            .class("h-full")
            .child_signal(_self.state.content_mode.signal().map(clone!(_self => move |content_mode| Some(
                Self::render_child(_self.clone(), content_mode)
            ))))
        })
    }

    fn render_child(_self: Rc<Self>, curr_content_mode: ContentMode) -> Dom {

        elem!(templates::sidebar_step_1(_self.game_mode, curr_content_mode), {
            .apply(|dom| apply_steps_nav(dom, _self.state.clone()))
            .apply_if(_self.game_mode == GameMode::WordsAndImages, 
                clone!(_self => move |dom| Self::apply_words_and_images(_self, dom, curr_content_mode))
            )
            .apply_if(curr_content_mode == ContentMode::TextInit, 
                clone!(_self => move |dom| Self::apply_text_init(_self, dom))
            )
            .apply_if(curr_content_mode == ContentMode::TextDone, 
                clone!(_self => move |dom| Self::apply_text_done(_self, dom))
            )
            .apply_if(curr_content_mode == ContentMode::Images, 
                clone!(_self => move |dom| Self::apply_images(_self, dom))
            )
        })
    }

    fn apply_words_and_images(_self: Rc<Self>, dom:DomBuilder<HtmlElement>, curr_content_mode: ContentMode) -> DomBuilder<HtmlElement> {
        apply_methods!(dom, {
            .with_data_id!("text-btn", {
                .event(clone!(_self, curr_content_mode => move |evt:events::Click| {
                    if curr_content_mode != ContentMode::TextInit {
                        _self.state.content_mode.set(ContentMode::TextDone);
                    }
                }))
            })
            .with_data_id!("images-btn", {
                .event(clone!(_self, curr_content_mode => move |evt:events::Click| {
                    if curr_content_mode != ContentMode::TextInit {
                        _self.state.content_mode.set(ContentMode::Images);
                    }
                }))
            })
        })
    }

    fn apply_text_init(_self: Rc<Self>, dom:DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        apply_methods!(dom, {
            .with_data_id!("list-items" => HtmlTextAreaElement, {
                .apply_if(*_self.state.first_text.borrow(), |dom| {
                    dom.property("value", crate::config::get_init_words_string())
                })
                .after_inserted(clone!(_self => move |elem| {
                    *_self.text_area.borrow_mut() = Some(elem);
                }))
            })
            .with_data_id!("done-btn", {
                .event(clone!(_self => move |evt:events::Click| {
                    if let Some(text_area) = _self.text_area.borrow().as_ref() {
                        *_self.state.first_text.borrow_mut() = false;
                        let pairs = make_pairs(_self.game_mode, text_area.value());
                        _self.state.pairs.lock_mut().replace_cloned(pairs);
                        _self.state.content_mode.set(ContentMode::TextDone);
                    }
                }))
            })
        })
    }
    fn apply_text_done(_self: Rc<Self>, dom:DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        apply_methods!(dom, {
            .with_data_id!("clear-btn", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.state.content_mode.set(ContentMode::TextInit)
                }))
            })
        })
    }
    fn apply_images(_self: Rc<Self>, dom:DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        apply_methods!(dom, {
            .with_data_id!("image-search", {
                .child(ImageSearchWidget::render(
                    ImageSearchWidget::new(
                        crate::debug::settings().image_search,
                        Some(clone!(_self => move |img:MetaImage| {
                        }))
                    )
                ))
            })
        })
    }
}

fn make_pairs(game_mode: GameMode, all_text:String) -> Vec<(Card, Card)> {
    all_text.lines()
        .map(|word| {
            match game_mode {
                GameMode::Duplicate => {
                    (
                        Card::new_with_data(CardMode::Text, word.to_string()),
                        Card::new_with_data(CardMode::Text, word.to_string()),
                    )
                },
                GameMode::WordsAndImages => {
                    (
                        Card::new_with_data(CardMode::Text, word.to_string()),
                        Card::new(CardMode::Image),
                    )
                }
            }
        })
        .collect()
}
