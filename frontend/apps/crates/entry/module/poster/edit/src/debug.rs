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
use shared::{
    domain::{
        jig::{
            module::body::{
                Image,
                ThemeChoice,
                Background,
                Instructions, 
                Transform,
                poster::{Content, Mode, Step, ModuleData as RawData},
                _groups::design::{BaseContent, Sticker, Text, Trace, TraceShape, Backgrounds, Sprite}
            },
            JigId, module::ModuleId
        },
        image::ImageId,
        audio::AudioId
    },
    media::MediaLibrary
};
use components::stickers::{sprite::ext::*, text::ext::*};
use components::tabs::MenuTabKind;

pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();

const IMAGE_UUID:&'static str = "e84dd7fe-c92d-11eb-8c82-cfd1d3fd13ff";


pub const DEBUG_TEXT:&'static str = "Hello World!";


#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data:Option<RawData>,
    pub step:Option<Step>,
    pub skip_save: bool,
    pub skip_load_jig: bool,
    pub bg_tab: Option<MenuTabKind>,
    pub content_tab: Option<MenuTabKind>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitData {
    pub stickers: Vec<InitSticker>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InitSticker {
    Text,
    Sprite,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InitTrace {
    //x, y, w, h
    Ellipse(f64, f64, f64, f64),
}

impl DebugSettings {
    pub fn debug(init_data: Option<InitData>) -> DebugSettings {
        DebugSettings {
            //debug always has to have some data
            //otherwise it will fail at load time
            data: Some(
                if let Some(init_data) = init_data {
                    RawData{
                        content: Some(Content {
                            mode: Mode::Poster,
                            base: BaseContent {
                                theme: ThemeChoice::Override(ThemeId::Chalkboard), 
                                instructions: Instructions::default(),
                                stickers: init_data.stickers.iter().map(|init| {
                                    match init {
                                        InitSticker::Text => {
                                            let value = components::text_editor::state::State::text_to_value(DEBUG_TEXT);
                                            let text = Text::new(value);
                                            Sticker::Text(text)
                                        },
                                        InitSticker::Sprite => Sticker::Sprite(Sprite::new(Image {
                                            id: ImageId(Uuid::parse_str(IMAGE_UUID).unwrap_ji()), 
                                            lib: MediaLibrary::Global
                                        }))
                                    }
                                }).collect(),
                                backgrounds: Backgrounds {
                                    layer_1: None, //Some(Background::Color(hex_to_rgba8("#ff0000"))),
                                    layer_2: None,
                                },
                            },
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
            skip_load_jig: true,
            bg_tab: Some(MenuTabKind::Image),
            content_tab: Some(MenuTabKind::Text),
        }
    }
}

pub fn init(jig_id: JigId, module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS.set(DebugSettings::debug(Some(InitData{
            stickers: vec![
                InitSticker::Text, //InitSticker::Sprite
            ],
        }))).unwrap_ji();
        
        //SETTINGS.set(DebugSettings::debug(None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
