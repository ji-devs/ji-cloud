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
    pub state: Rc<RefCell<Option<BaseGameState>>>,
}


impl GameState {
    pub fn new(jig_id:String, module_id: String) -> Self {
        Self {
            jig_id,
            module_id,
            mode: Mutable::new(None),
            state: Rc::new(RefCell::new(None))
        }
    }

    pub fn set_from_loaded(&self, step: usize, raw:GameStateRaw) {
        if self.mode.get().is_some() {
            panic!("setting the game state from loaded only works on first-load!");
        }

        let (mode, state) = match raw {
            GameStateRaw::Duplicate(raw_state) => {
                (
                    Some(GameMode::Duplicate),
                    Some(BaseGameState::from_raw(step, raw_state, self.jig_id.clone(), self.module_id.clone()))
                )
            },
            GameStateRaw::WordsAndImages(raw_state) => {
                (
                    Some(GameMode::WordsAndImages),
                    Some(BaseGameState::from_raw(step, raw_state, self.jig_id.clone(), self.module_id.clone()))
                )
            },
            GameStateRaw::None => (None, None),
            _ => unimplemented!("no way to load {:?}", raw) 
        };

        //Note that this will *not* trigger re-renders of the inner mode pages
        //Using set_from_loaded for runtime changes is therefore very inefficient!
        //It's only meant for first-time loading
        *self.state.borrow_mut() = state;
        //wrapped in a Some because None here means "loading"
        //this *will* trigger re-renders of everything from the top-level
        //an inner none means "loaded but no mode"
        self.mode.set(Some(mode));
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameMode {
    Duplicate,
    WordsAndImages,
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
pub struct BaseGameState {
    pub jig_id: String,
    pub module_id: String,
    pub step: Mutable<Step>,
    pub cards: MutableVec<Card>,
    pub theme_id: Mutable<String>,
}

impl BaseGameState {
    pub fn to_raw(&self) -> BaseGameStateRaw {
        BaseGameStateRaw {
            cards: self.cards.lock_ref().iter().map(|card| card.into()).collect(),
            theme_id: self.theme_id.get_cloned(),
        }
    }

    pub fn from_raw(step: usize, raw: BaseGameStateRaw, jig_id: String, module_id: String) -> Self {
        let cards:Vec<Card> = raw.cards
            .into_iter()
            .map(|raw_card| raw_card.into())
            .collect();

        Self {
            jig_id,
            module_id,
            step: Mutable::new(step.into()),
            cards: MutableVec::new_with_values(cards),
            theme_id: Mutable::new(raw.theme_id)
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
