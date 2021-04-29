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
use uuid::Uuid;
use shared::{
    domain::{
        jig::{
            module::body::{Sprite, Instructions},
            JigId, module::ModuleId
        },
        image::ImageId,
        audio::AudioId
    },
    media::MediaLibrary
};

pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();
const STRING_UUID:&'static str = "bf2fe548-7ffd-11eb-b3ab-579026da8b36";

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
    pub fn debug(with_data: bool) -> DebugSettings {
        DebugSettings {
            data: Some(
                if with_data {
                    raw::ModuleData{
                        theme_id: ThemeId::Chalkboard, 
                        instructions: Instructions::default(),
                        stickers: vec![
                            Sprite::new(ImageId(Uuid::parse_str(STRING_UUID).unwrap_ji()), MediaLibrary::Global)
                        ],
                        ..raw::ModuleData::default()
                    }
                } else {
                    raw::ModuleData{
                        theme_id: ThemeId::Chalkboard, 
                        instructions: Instructions::default(),
                        ..raw::ModuleData::default()
                    }
                }
            ),
            step: Some(Step::One), 
            live_save: false,
            content_tab: Some(DebugContentTab::Images),
        }
    }
}

pub fn init(jig_id: JigId, module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS.set(DebugSettings::debug(true)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
