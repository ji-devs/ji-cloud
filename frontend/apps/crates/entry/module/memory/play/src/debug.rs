use cfg_if::cfg_if;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};

use std::cell::RefCell;
use std::rc::Rc;
use crate::data::raw;

pub const DEBUG_IMAGE_ID:&'static str ="6468777e-2008-11eb-a943-331f3eea16f5";

pub struct DebugSettings {
    pub data:Option<raw::GameData>,
    pub shuffle: bool,
    pub ending: bool,
}

impl DebugSettings {
    pub fn local() -> DebugSettings {
        DebugSettings {
            data: Some(raw::GameData::duplicate_debug(
                crate::config::get_init_words_iter(),
                crate::config::get_themes_cloned()[1].clone()
            )),
            shuffle: false,
            ending: true,
        }
    }
    pub fn default() -> DebugSettings {
        DebugSettings {
            data: None,
            shuffle: true,
            ending: false,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn settings() -> DebugSettings {
            //DebugSettings::local()
            DebugSettings::default()
        }
    } else {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    }
}

