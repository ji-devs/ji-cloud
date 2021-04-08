use crate::data::state::*;
use cfg_if::cfg_if;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;
use crate::data::{raw, state::*};
use once_cell::sync::OnceCell;
use utils::prelude::*;

pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();

#[derive(Debug)]
pub struct DebugSettings {
    pub data:Option<Option<raw::GameData>>,
    pub step:Option<Step>,
    pub live_save: bool,
    pub content_tab: Option<DebugContentTab>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebugContentTab {
    Text,
    Images
}


impl DebugSettings {
    pub fn default() -> DebugSettings {
        DebugSettings {
            data: None, 
            step: None, 
            live_save: true,
            content_tab: None,
        }
    }
    pub fn debug(mode: Option<raw::Mode>, with_data: bool) -> DebugSettings {
        DebugSettings {
            data: Some(
                mode.map(|mode| {
                    raw::GameData::new(
                        mode, 
                        ThemeId::Chalkboard, 
                        raw::Instructions::new(),
                        {
                            if with_data {
                                //vec![("foo", "foo")]
                                crate::config::get_debug_pairs(mode)
                            } else {
                                Vec::new()
                            }
                        }
                    )
                })
            ),
            step: Some(Step::One), 
            live_save: false,
            content_tab: Some(DebugContentTab::Text),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn init() {
            SETTINGS.set(DebugSettings::debug(Some(GameMode::BeginsWith), false)).unwrap_ji();
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
