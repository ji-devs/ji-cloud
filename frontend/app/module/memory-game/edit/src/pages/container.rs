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
use crate::data::{*, raw::*};
use super::mode_choose::ModeChoosePage;
use super::duplicate::container::DuplicatePage;
use crate::debug;

pub struct ContainerPage {
    pub state: GameState,
    pub loader: AsyncLoader,
}

impl ContainerPage {
    pub fn new() -> Rc<Self> {
        let _self_clone = Rc::new(Self { 
            state:  GameState::new(),
            loader: AsyncLoader::new(),
            //game_mode: Mutable::new(debug::settings().game_mode.unwrap_or(None)),
        });

        let _self = _self_clone.clone();

        _self_clone.loader.load(async move {

            if let Some(raw_state) = debug::settings().state {
                _self.state.set_from_loaded(raw_state);
            } else {
                //TODO - LOAD GAME STATE FROM BACKEND
                log::info!("loading...");
                let raw_state:GameStateRaw = GameStateRaw::load().await;
                _self.state.set_from_loaded(raw_state);
            }
        });
    

        _self_clone
    }
    
    fn dom_signal(_self:Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        _self.state.mode.signal_ref(clone!(_self => move |mode| {
            match mode {
                //This level of none means we're still loading the state
                None => None,
                Some(mode) => {
                    log::info!("{:?}", mode);
                    match mode {
                        //This level of none means we've loaded but it's initial screen
                        None => {
                            Some(ModeChoosePage::render(ModeChoosePage::new(clone!(_self => move |mode| {
                                match mode {
                                    GameMode::Duplicate => {
                                        let mode_state:DuplicateState = DuplicateStateRaw::default().into(); 
                                        *_self.state.mode_state.borrow_mut() = Some(ModeState::Duplicate(Rc::new(mode_state)));
                                    }
                                }

                                _self.state.mode.set(Some(Some(mode)));
                            }))))
                        },
                        Some(mode) => {
                            let mode_state:&Option<ModeState> = &_self.state.mode_state.borrow_mut();
                            let mode_state:&ModeState = &mode_state.as_ref().expect_throw("mode without mode_state is a bug!!");
                            match mode_state {
                                ModeState::Duplicate(mode_state) => {
                                    Some(DuplicatePage::render(DuplicatePage::new(mode_state.clone())))
                                }
                                _ => None
                            }
                        }
                    }
                }
            }
        }))
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", { .child_signal(Self::dom_signal(_self.clone())) } )
    }
}
