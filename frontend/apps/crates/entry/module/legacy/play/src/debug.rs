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
use utils::{prelude::*, colors::*};
use uuid::Uuid;
use shared::{domain::{audio::AudioId, image::ImageId, jig::{JigId, module::{ModuleCreateRequest, ModuleId, ModuleBody, body::{
                    Image,
                    ThemeChoice,
                    Background, 
                    Instructions, Transform,
                    legacy::{
                        ModuleData as RawData
                    },
                    _groups::design::{Backgrounds,Sprite, Sticker, Text, Trace,BaseContent }
                }}}}, media::MediaLibrary};
use awsm_web::loaders::fetch::fetch_url;

pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();

const IMAGE_UUID: &'static str = "e84dd7fe-c92d-11eb-8c82-cfd1d3fd13ff";

pub const DEBUG_TEXT: &'static str = "Hello World!";

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data: Option<RawData>,
    pub skip_load_jig: bool,
}

impl DebugSettings {
    pub fn debug(data: RawData) -> DebugSettings {
        DebugSettings {
            data: Some(data),
            skip_load_jig: true,
        }
    }
}

pub async fn init(jig_id: JigId, _module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        // http://localhost:4104/module/legacy/play/debug?game_id=web-stress-test&slide_index=0&example=true
        let data = match utils::routes::get_param("game_id") {
            Some(game_id) => {
                let slide_index = utils::routes::get_param_index("slide_index").unwrap_or_default();
                let url = utils::path::legacy_cdn_url(format!("{}/json/requests/module-{}.json", game_id, slide_index));

                let module_req:ModuleCreateRequest = fetch_url(&url)
                    .await
                    .unwrap_ji()
                    .json_from_str()
                    .await
                    .unwrap_ji();

                let slide_id = match module_req.body {
                    ModuleBody::Legacy(legacy) => legacy.slide_id,
                    _ => panic!("not a legacy module?!")
                };

                RawData{
                    game_id,
                    slide_id
                }
                
            },
            None => {
                RawData::default()
            } 
        };

        SETTINGS.set(DebugSettings::debug(data)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
