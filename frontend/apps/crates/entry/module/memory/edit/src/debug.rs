use crate::data::*;
use cfg_if::cfg_if;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};

use std::cell::RefCell;
use std::rc::Rc;
use crate::data::*; 
use crate::config::BaseGameStateExt;


pub const DEBUG_STEP:usize = 2;
pub const DEBUG_THEME_INDEX:usize = 0;

pub const DEBUG_PLAY_CARD_TEXTS:&[&'static str] = &[
    "שמש",
    "ירח",
    "כוכב",
    "blah",
    "foo",
    "Sun",
    "Moon",
    "Star",
];



#[derive(Default)]
pub struct DebugSettings {
    pub state:Option<raw::GameState>,
    pub step:Option<usize>,
}

impl DebugSettings {
    pub fn words_and_images() -> Self {
        Self {
            state: Some(raw::GameState::WordsAndImages(
                raw::BaseGameState::default_words_and_images()
            )),
            step: Some(DEBUG_STEP),
        }
    }

    pub fn duplicate() -> Self {
        Self {
            state: Some(raw::GameState::Duplicate(
                raw::BaseGameState::default_duplicate()
            )),
            step: Some(DEBUG_STEP),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn settings() -> DebugSettings {
            DebugSettings::words_and_images()
        }
    } else {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    }
}

