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

pub const DEBUG_IMAGE_ID:&'static str ="6468777e-2008-11eb-a943-331f3eea16f5";

#[derive(Default)]
pub struct DebugSettings {
    pub state:Option<raw::GameState>,
    pub step:Option<usize>,
    pub shuffle: bool,
}

impl DebugSettings {
    pub fn local() -> Self {
        Self {
            state: Some(raw::GameState::Duplicate(
                raw::BaseGameState {               
                    pairs: DEBUG_PLAY_CARD_TEXTS 
                        .iter()
                        .enumerate()
                        .map(|(index, text)| {
                            (
                                raw::Card::Text(text.to_string()),
                                if index == 0 {
                                    raw::Card::Image(Some(DEBUG_IMAGE_ID.to_string()))
                                } else if index == 1 {
                                    raw::Card::Image(None)
                                } else {
                                    raw::Card::Text(text.to_string())
                                }
                            )
                        })
                        .collect(),
                    theme_id: "basic".to_string(),
                },
            )),
            step: Some(DEBUG_STEP),
            shuffle: false,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn settings() -> DebugSettings {
            DebugSettings::local()
        }
    } else {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    }
}

