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
use crate::data::raw::*;

#[derive(Default)]
pub struct DebugSettings {
    pub state:Option<GameStateRaw>,
    pub step:Option<usize>,
}

impl DebugSettings {
    pub fn local() -> Self {
        Self {
            state: Some(GameStateRaw::debug()),
            step: Some(crate::config::DEBUG_STEP),
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

