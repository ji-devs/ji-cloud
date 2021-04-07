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

pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();

#[derive(Debug)]
pub struct DebugSettings {
    pub data:Option<Option<raw::GameData>>,
    pub step:Option<Step>,
    //just used for words and images, but whatever
    pub image_search: Option<()>,
    pub live_save: bool,
}

impl DebugSettings {
    pub fn default() -> DebugSettings {
        DebugSettings {
            data: None, 
            step: None, 
            image_search: None,
            live_save: true,
        }
    }
    pub fn debug(mode: Option<raw::Mode>, with_data: bool) -> DebugSettings {
        DebugSettings {
            data: Some(
                mode.map(|mode| {
                    raw::GameData::new(
                        mode, 
                        crate::config::get_themes_cloned()[1].clone(),
                        raw::Instructions::new(),
                        {
                            if with_data {
                                //vec![("foo", "foo")]
                                crate::config::get_init_words(mode)
                            } else {
                                Vec::new()
                            }
                        }
                    )
                })
            ),
            step: Some(Step::Three), 
            image_search: None,
            live_save: false,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn init() {
            SETTINGS.set(DebugSettings::debug(Some(GameMode::WordsAndImages), false)).unwrap_throw();
            //SETTINGS.set(DebugSettings::debug(Some(GameMode::BeginsWith), false)).unwrap_throw();
            //SETTINGS.set(DebugSettings::debug(Some(GameMode::Lettering), false)).unwrap_throw();
            //SETTINGS.set(DebugSettings::debug(Some(GameMode::Duplicate), false)).unwrap_throw();
            //SETTINGS.set(DebugSettings::debug(None, false)).unwrap_throw();
        }

        pub fn settings() -> &'static DebugSettings {
            unsafe { SETTINGS.get_unchecked() }
        }
    } else {
        pub fn init() {
            SETTINGS.set(DebugSettings::default()).unwrap_throw();
        }
        pub fn settings() -> &'static DebugSettings {
            unsafe { SETTINGS.get_unchecked() }
        }
    }
}
