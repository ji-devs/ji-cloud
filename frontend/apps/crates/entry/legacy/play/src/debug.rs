use cfg_if::cfg_if;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct DebugSettings {
    pub is_mock: bool
}

impl DebugSettings {
    pub fn local() -> Self {
        Self {
            is_mock: true
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

