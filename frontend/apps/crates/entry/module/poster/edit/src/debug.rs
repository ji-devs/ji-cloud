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


pub const DEBUG_STEP:usize = 1;
pub const DEBUG_THEME_INDEX:usize = 0;

pub struct DebugSettings {
    pub poster:Option<raw::Poster>,
}

impl DebugSettings {
    pub fn default() -> DebugSettings {
        DebugSettings {
            poster: None, 
        }
    }

    pub fn theme() -> Self {
        Self {
            poster: None,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    } else {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    }
}

