use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use super::mode_choose::ModeChoosePage;
use super::duplicate::container::DuplicatePage;
use crate::debug;

pub struct ContainerPage {
    pub game_mode: Mutable<Option<GameMode>>
}

impl ContainerPage {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            game_mode: Mutable::new(debug::settings().game_mode.unwrap_or(None)),
        });

        _self
    }
    
    fn dom_signal(_self:Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        _self.game_mode.signal_ref(clone!(_self => move |mode| {
            match mode {
                None => Some(ModeChoosePage::render(ModeChoosePage::new(clone!(_self => move |mode| {
                    _self.game_mode.set(Some(mode));
                })))),
                Some(mode) => match mode {
                    GameMode::Duplicate => Some(DuplicatePage::render(DuplicatePage::new())),
                    _ => None,
                }
                
            }
        }))
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", { .child_signal(Self::dom_signal(_self.clone())) } )
    }
}
