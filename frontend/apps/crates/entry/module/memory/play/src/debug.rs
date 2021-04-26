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
use shared::domain::jig::{JigId, module::ModuleId};
use uuid::Uuid;
use shared::domain::jig::module::body::Instructions;

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
                    Instructions::default(),
                    crate::config::get_debug_pairs(mode, 3)
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


pub fn init(jig_id: JigId, module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS.set(DebugSettings::debug(Mode::Lettering)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}
pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
