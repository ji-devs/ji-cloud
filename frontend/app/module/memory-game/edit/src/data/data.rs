use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::debug;
use super::raw::*;
use itertools::Itertools;
use std::fmt::Write;

pub struct GameState {
    pub jig_id: String,
    pub module_id: String,
    //outer option is for "loading", inner option is for "no module chosen"
    pub mode: Mutable<Option<Option<GameMode>>>, 
    pub mode_state: Rc<RefCell<Option<ModeState>>>,
}


impl GameState {
    pub fn new(jig_id:String, module_id: String) -> Self {
        Self {
            jig_id,
            module_id,
            mode: Mutable::new(None),
            mode_state: Rc::new(RefCell::new(None))
        }
    }

    pub fn set_from_loaded(&self, raw:GameStateRaw) {
        if self.mode.get().is_some() {
            panic!("setting the game state from loaded only works on first-load!");
        }

        let mode:Option<GameMode> = 
            raw.mode.map(|raw_mode| raw_mode.into());

        let mode_state:Option<ModeState> = {
            raw.mode_state.map(|raw_mode_state| match raw_mode_state {
                ModeStateRaw::Duplicate(raw_state) => {
                    ModeState::Duplicate(Rc::new(raw_state.into_mutable(self.jig_id.clone(), self.module_id.clone())))
                }
            })
        };

        //Note that this will *not* trigger re-renders of the inner mode pages
        //Using set_from_loaded for runtime changes is therefore very inefficient!
        //It's only meant for first-time loading
        *self.mode_state.borrow_mut() = mode_state;
        //wrapped in a Some because None here means "loading"
        //this *will* trigger re-renders of everything from the top-level
        self.mode.set(Some(mode));
    }

    pub fn to_raw(&self) -> GameStateRaw {
        let mode = self.mode.get();
        let mode = mode.expect_throw("can't convert to raw while loading");

        match mode {
            None => GameStateRaw { mode: None, mode_state: None },
            Some(_) => {
                let mode_state = self.mode_state.borrow();
                let mode_state = mode_state.as_ref().expect_throw("need mode state to convert to raw");
                match mode_state {
                    _ => unimplemented!("todo!")
                }
            }
        }


    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameMode {
    Duplicate
}

impl From<GameModeRaw> for GameMode {
    fn from(mode:GameModeRaw) -> Self {
        match mode {
            GameModeRaw::Duplicate => Self::Duplicate
        }
    }
}
#[derive(Debug)]
pub enum ModeState {
    Duplicate(Rc<DuplicateState>)
}

#[derive(Clone, Debug)]
pub struct Card {
    pub text: Mutable<String>,
}
impl From<String> for Card {
    fn from(text:String) -> Self {
        Self {text: Mutable::new(text) }
    }
}
impl From<CardRaw> for Card {
    fn from(card:CardRaw) -> Self {
        Self {text: Mutable::new(card.text) }
    }
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub id: &'static str,
    pub label: &'static str,
}

#[derive(Debug)]
pub struct DuplicateState {
    pub jig_id: String,
    pub module_id: String,
    pub step: Mutable<Step>,
    pub cards: MutableVec<Card>,
    pub theme_id: Mutable<String>,
}

impl DuplicateStateRaw {
    pub fn into_mutable(self, jig_id: String, module_id: String) -> DuplicateState {
        let cards:Vec<Card> = self.cards
            .into_iter()
            .map(|raw_card| raw_card.into())
            .collect();

        DuplicateState {
            jig_id,
            module_id,
            step: Mutable::new(self.step.into()),
            cards: MutableVec::new_with_values(cards),
            theme_id: Mutable::new(self.theme_id)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Step {
    One,
    Two,
    Three,
    Four
}

impl From<StepRaw> for Step {
    fn from(step:StepRaw) -> Self {
        match step {
            StepRaw::One => Self::One,
            StepRaw::Two => Self::Two,
            StepRaw::Three => Self::Three,
            StepRaw::Four => Self::Four,
        }
    }
}
