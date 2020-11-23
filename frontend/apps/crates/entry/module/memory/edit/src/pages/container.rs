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
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::config::BaseGameStateExt;
use super::mode_choose::ModeChoosePage;
use crate::debug;

pub struct ContainerPage {
    pub game_state: GameState,
    pub loader: AsyncLoader,
}

impl ContainerPage {
    pub fn new(jig_id: String, module_id: String) -> Rc<Self> {
        let _self_clone = Rc::new(Self { 
            game_state:  GameState::new(jig_id, module_id),
            loader: AsyncLoader::new(),
            //game_mode: Mutable::new(debug::settings().game_mode.unwrap_or(None)),
        });

        let _self = _self_clone.clone();

        _self_clone.loader.load(async move {

            if let Some(raw_game_state) = debug::settings().state {
                _self.game_state.set_from_loaded(debug::settings().step.unwrap_or(1), raw_game_state);
            } else {
                //TODO - LOAD GAME STATE FROM BACKEND
                log::info!("loading...");
                let raw_state = raw::GameState::load(_self.game_state.jig_id.clone(), _self.game_state.module_id.clone()).await;
                _self.game_state.set_from_loaded(1, raw_state);
            }
        });
    

        _self_clone
    }
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", { 
            .child_signal(
                _self.game_state.mode.signal_ref(clone!(_self => move |mode| {
                    match mode {
                        //This level of none means we're still loading the state
                        None => None,
                        Some(mode) => {
                            match mode {
                                //This level of none means we've loaded but it's initial screen
                                None => Some(Self::render_choose(_self.clone())),
                                Some(mode) => Some(Self::render_mode(_self.clone(), *mode)),
                            }
                        }
                    }
                }))
            )
        })
    }

    fn render_mode(_self:Rc<Self>, mode: GameMode) -> Dom {

        let state:Rc<BaseGameState> = Rc::new(_self.game_state.state.borrow_mut().take().unwrap_throw());

        html!("div", { 
            .child_signal(state.clone().step.signal().map(move |step| {
                match step {
                    Step::One => {
                        match mode {
                            GameMode::Duplicate => {
                                Some(super::duplicate::Step1Page::render(
                                    super::duplicate::Step1Page::new(state.clone())
                                ))
                            },
                            GameMode::WordsAndImages => {
                                Some(super::words_and_images::Step1Page::render(
                                    super::words_and_images::Step1Page::new(state.clone())
                                ))
                            },
                            _ => None
                        }
                    },
                    Step::Two => Some(super::all_modes::step_2::Step2Page::render(
                            super::all_modes::step_2::Step2Page::new(state.clone()), mode)
                    ),
                    Step::Four => Some(super::all_modes::step_4::Step4Page::render(
                            super::all_modes::step_4::Step4Page::new(state.clone()), mode)
                    ),
                    _ => None
                }
            }))
        })
    }
    fn render_choose(_self:Rc<Self>) -> Dom {
        ModeChoosePage::render(ModeChoosePage::new(clone!(_self => move |mode| {

            *_self.game_state.state.borrow_mut() = Some(
                BaseGameState::from_raw(
                    1,
                    mode,
                    match mode {
                        GameMode::Duplicate => {
                            raw::BaseGameState::default_duplicate()
                        },
                        GameMode::WordsAndImages => {
                            raw::BaseGameState::default_words_and_images()
                        }
                    },
                    _self.game_state.jig_id.clone(), 
                    _self.game_state.module_id.clone()
                )
            );

            _self.game_state.mode.set(Some(Some(mode)));
        })))
    }
    
}
