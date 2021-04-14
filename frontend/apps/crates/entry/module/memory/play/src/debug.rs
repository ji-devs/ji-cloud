use cfg_if::cfg_if;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use std::cell::RefCell;
use std::rc::Rc;
use crate::data::{state::*, raw};
use once_cell::sync::OnceCell;
use utils::prelude::*;

pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();

pub const DEBUG_IMAGE_ID:&'static str ="6468777e-2008-11eb-a943-331f3eea16f5";

#[derive(Debug)]
pub struct DebugSettings {
    pub data:Option<raw::ModuleData>,
    pub shuffle: bool,
    pub ending: bool,
}

impl DebugSettings {
    pub fn debug(mode: raw::Mode) -> DebugSettings {
        DebugSettings {
            data: Some(
                    raw::ModuleData::new(
                        mode, 
                        ThemeId::Chalkboard, 
                        raw::Instructions::new(),
                        crate::config::get_debug_pairs(mode)
                    )
            ),
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
        pub fn init() {
            SETTINGS.set(DebugSettings::debug(GameMode::Duplicate)).unwrap_ji();
        }

        pub fn settings() -> &'static DebugSettings {
            unsafe { SETTINGS.get_unchecked() }
        }
    } else {
        pub fn init() {
            SETTINGS.set(DebugSettings::default()).unwrap_ji();
        }
        pub fn settings() -> &'static DebugSettings {
            unsafe { SETTINGS.get_unchecked() }
        }
    }
}
