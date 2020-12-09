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
use super::steps_nav::apply_steps_nav;
use wasm_bindgen::JsCast;

pub struct Step2Sidebar {
    state: Rc<State>, 
    game_mode: GameMode,
}

impl Step2Sidebar {
    pub fn new(state: Rc<State>, game_mode:GameMode) -> Rc<Self> {
        Rc::new(Self { 
            state, 
            game_mode,
        })
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::sidebar_step_2(), {
            .apply(|dom| apply_steps_nav(dom, _self.state.clone()))
            .with_data_id!("theme-items", {
                .children(crate::config::get_themes_iter()
                          .map(clone!(_self => move |theme| {
                              ThemeDom::render(ThemeDom::new(_self.state.clone(), theme.clone()))
                          }))
                          .collect::<Vec<Dom>>()
                )
            })
        })
    }
}

pub struct ThemeDom {
    pub state: Rc<State>,
    pub is_hover:Mutable<bool>,
    pub theme: Theme,
}

impl ThemeDom {
    pub fn new(state:Rc<State>, theme: Theme) -> Rc<Self> {
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
                    crate::config::THEME_EXAMPLE_TEXT_2
                } else {
                    crate::config::THEME_EXAMPLE_TEXT_1
                }
            })
    }
    pub fn render(_self: Rc<Self>) -> Dom { 
        html!("div", {
            .class("w-full")
            .class("h-full")
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
                                    .text(&_self.theme.label)
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
