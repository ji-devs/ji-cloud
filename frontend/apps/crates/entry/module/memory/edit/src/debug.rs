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
use shared::domain::jig::{JigId, module::ModuleId};
use uuid::Uuid;
use shared::domain::jig::module::body::Instructions as RawInstructions;

pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();

#[derive(Debug)]
pub struct DebugSettings {
    pub data:Option<raw::ModuleData>,
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
                if with_data {
                    let mode = mode.unwrap_ji();
                    raw::ModuleData::new(
                        mode, 
                        ThemeId::Chalkboard, 
                        RawInstructions::default(),
                        crate::config::get_debug_pairs(mode)
                    )
                } else {
                    raw::ModuleData{
                        mode,
                        theme_id: ThemeId::Chalkboard,
                        ..raw::ModuleData::default()
                    }
                }
            ),
            step: Some(Step::One), 
            live_save: false,
            content_tab: Some(DebugContentTab::Text),
        }
    }
}

pub fn init(jig_id: JigId, module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        //SETTINGS.set(DebugSettings::debug(Some(Mode::Lettering), true)).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(Some(Mode::BeginsWith), false)).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(Some(Mode::Riddles), false)).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(Some(Mode::Duplicate), true)).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(None, false)).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(Some(Mode::Duplicate), false)).unwrap_ji();
        SETTINGS.set(DebugSettings::debug(Some(Mode::WordsAndImages), true)).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(Some(Mode::BeginsWith), false)).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(Some(Mode::Riddles), false)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
