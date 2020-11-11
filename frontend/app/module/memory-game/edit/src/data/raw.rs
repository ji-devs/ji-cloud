use serde::{Serialize, Deserialize};
use super::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameStateRaw {
    pub mode: Option<GameModeRaw>,
    pub mode_state: Option<ModeStateRaw> 
}

impl GameStateRaw {
    pub async fn load() -> Self {
        Self {
            mode: None, 
            mode_state: None 
        }
    }

    pub fn debug() -> Self {
        Self {
            mode: Some(GameModeRaw::Duplicate),
            mode_state: Some(ModeStateRaw::Duplicate(DuplicateStateRaw::debug())),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum GameModeRaw {
    Duplicate
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ModeStateRaw {
    Duplicate(DuplicateStateRaw)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardRaw {
    pub text: String,
}
impl CardRaw {
    pub fn new(text:String) -> Self {
        Self {
            text 
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DuplicateStateRaw {
    pub step: StepRaw,
    pub cards: Vec<CardRaw>,
}

impl DuplicateStateRaw {
    pub fn default() -> Self {
        Self {
            step: StepRaw::One,
            cards: vec![
                "שמש",
                "ירח",
                "כוכב",
                "Sun",
                "Moon",
                "Star",
            ].into_iter()
                .map(|text| {
                    CardRaw::new(text.to_string())
                })
                .collect()
        }
    }
    pub fn debug() -> Self {
        Self {
            step: StepRaw::One,
            cards: vec![
                "שמש",
                "ירח",
                "כוכב",
                "Sun",
                "Moon",
                "Star",
            ].into_iter()
                .map(|text| {
                    CardRaw::new(text.to_string())
                })
                .collect()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StepRaw {
    One,
    Two,
    Three,
    Four
}

