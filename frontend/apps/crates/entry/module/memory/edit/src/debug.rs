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


pub const DEBUG_STEP:usize = 1;
pub const DEBUG_THEME_INDEX:usize = 0;

pub struct DebugSettings {
    pub state:Option<raw::GameState>,
    pub step:Option<usize>,
    //just used for words and images, but whatever
    pub content_mode: ContentMode,
}

impl DebugSettings {
    pub fn words_and_images() -> Self {
        Self {
            state: Some(raw::GameState::WordsAndImages(
                raw::BaseGameState::default_words_and_images()
            )),
            step: Some(DEBUG_STEP),
            content_mode: ContentMode::Images,
        }
    }

    pub fn duplicate() -> Self {
        Self {
            state: Some(raw::GameState::Duplicate(
                raw::BaseGameState::default_duplicate()
            )),
            step: Some(DEBUG_STEP),
            content_mode: ContentMode::Text,
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
            Self {
                state: None, 
                step: None, 
                content_mode: ContentMode::Text,
            }
        }
    }
}

