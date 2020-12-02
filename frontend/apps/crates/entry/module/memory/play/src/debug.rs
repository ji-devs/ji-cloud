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


pub const DEBUG_IMAGE_ID:&'static str ="6468777e-2008-11eb-a943-331f3eea16f5";

pub struct DebugSettings {
    pub data:Option<raw::GameData>,
    pub shuffle: bool,
}

impl DebugSettings {
    pub fn local() -> DebugSettings {
        DebugSettings {
            data: Some(raw::GameData::duplicate_debug(
                crate::config::get_init_words_iter(),
                crate::config::get_themes_cloned()[0].id.clone()
            )),
            shuffle: false,
        }
    }
    pub fn default() -> DebugSettings {
        DebugSettings {
            data: None,
            shuffle: true,
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

