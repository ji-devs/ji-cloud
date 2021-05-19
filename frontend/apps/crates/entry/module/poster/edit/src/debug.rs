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
use shared::domain::jig::module::body::poster::{Content, Mode as RawMode, ModuleData as RawData};
use crate::state::Mode;
use crate::steps::state::Step;

pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();

#[derive(Debug)]
pub struct DebugSettings {
    pub data:Option<RawData>,
    pub step:Option<Step>,
    pub skip_save: bool,
}


impl DebugSettings {
    pub fn default() -> DebugSettings {
        DebugSettings {
            data: None, 
            step: None, 
            skip_save: false,
        }
    }
    pub fn debug(with_data: bool) -> DebugSettings {
        DebugSettings {
            //debug always has to have some data
            //otherwise it will fail at load time
            data: Some(
                if with_data {
                    RawData{
                        content: Some(Content {
                            mode: RawMode::Poster,
                            theme_id: ThemeId::Chalkboard, 
                            instructions: Instructions::default(),
                            ..Content::default()
                        })
                    }
                } else {
                    RawData{
                        content: None                    
                    }
                }
            ),
            step: Some(Step::One),
            skip_save: true,
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
