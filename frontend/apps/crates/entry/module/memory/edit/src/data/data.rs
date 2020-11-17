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

pub trait ModeStateExt {
    type MutableContainer;
    fn into_mutable(self, step:usize, jig_id: String, module_id: String) -> Self::MutableContainer;
}

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

    pub fn set_from_loaded(&self, step: usize, raw:GameStateRaw) {
        if self.mode.get().is_some() {
            panic!("setting the game state from loaded only works on first-load!");
        }

        let mode:Option<GameMode> = 
            raw.mode.map(|raw_mode| raw_mode.into());

        let mode_state:Option<ModeState> = {
            raw.mode_state.map(|raw_mode_state| match raw_mode_state {
                ModeStateRaw::Duplicate(raw_state) => {
                    ModeState::Duplicate(Rc::new(raw_state.into_mutable(step, self.jig_id.clone(), self.module_id.clone())))
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
}

impl From<&DuplicateState> for GameStateRaw {
    fn from(state:&DuplicateState) -> Self {
        Self {
            mode: Some(GameModeRaw::Duplicate),
            mode_state: Some(ModeStateRaw::Duplicate(state.into()))
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
impl From<&Card> for CardRaw {
    fn from(card:&Card) -> Self {
        Self {text: card.text.get_cloned() }
    }
}


#[derive(Debug)]
pub struct DuplicateState {
    pub jig_id: String,
    pub module_id: String,
    pub step: Mutable<Step>,
    pub cards: MutableVec<Card>,
    pub theme_id: Mutable<String>,
}
impl From<&DuplicateState> for DuplicateStateRaw {
    fn from(state:&DuplicateState) -> Self {
        Self {
            cards: state.cards.lock_ref().iter().map(|card| card.into()).collect(),
            theme_id: state.theme_id.get_cloned(),
        }
    }
}

impl ModeStateExt for DuplicateStateRaw {
    type MutableContainer = DuplicateState;

    fn into_mutable(self, step: usize, jig_id: String, module_id: String) -> DuplicateState {
        let cards:Vec<Card> = self.cards
            .into_iter()
            .map(|raw_card| raw_card.into())
            .collect();

        DuplicateState {
            jig_id,
            module_id,
            step: Mutable::new(step.into()),
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

impl From<usize> for Step {
    fn from(step:usize) -> Self {
        match step {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            _ => panic!("unallowed step!") 
        }
    }
}
