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
use crate::pages::all_modes::{
    steps_nav::apply_steps_nav,
    card_dom::apply_preview_cards,
};
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
            .apply(|dom| apply_preview_cards(dom, _self.state.clone()))
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




