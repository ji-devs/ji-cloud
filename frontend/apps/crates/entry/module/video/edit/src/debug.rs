#![allow(dead_code)]

use components::stickers::{sprite::ext::*, text::ext::*};
use components::tabs::MenuTabKind;
use once_cell::sync::OnceCell;
use shared::{
    domain::{
        image::ImageId,
        jig::{
            module::body::{
                Image, Instructions, ThemeChoice,
                _groups::design::{Backgrounds, BaseContent, Sprite, Sticker, Text},
                video::{Content, Mode, ModuleData as RawData, Step},
            },
            module::ModuleId,
            JigId,
        },
    },
    media::MediaLibrary,
};
use utils::prelude::*;
use uuid::Uuid;

pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();

const IMAGE_UUID: &str = "e84dd7fe-c92d-11eb-8c82-cfd1d3fd13ff";

pub const DEBUG_TEXT:&str = "{\"version\":\"0.1.0\",\"content\":[{\"children\":[{\"text\":\"text from rust\",\"element\":\"P1\"}]}]}";

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data: Option<RawData>,
    pub step: Option<Step>,
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
            data: Some(if let Some(init_data) = init_data {
                RawData {
                    content: Some(Content {
                        mode: Mode::Introduction,
                        base: BaseContent {
                            theme: ThemeChoice::Override(ThemeId::Chalkboard),
                            instructions: Instructions::default(),
                            stickers: init_data
                                .stickers
                                .iter()
                                .map(|init| match init {
                                    InitSticker::Text => {
                                        Sticker::Text(Text::new(DEBUG_TEXT.to_string()))
                                    }
                                    InitSticker::Sprite => Sticker::Sprite(Sprite::new(Image {
                                        id: ImageId(Uuid::parse_str(IMAGE_UUID).unwrap_ji()),
                                        lib: MediaLibrary::Global,
                                    })),
                                })
                                .collect(),
                            backgrounds: Backgrounds {
                                layer_1: None, //Some(Background::Color(hex_to_rgba8("#ff0000"))),
                                layer_2: None,
                            },
                        },
                        ..Content::default()
                    }),
                }
            } else {
                RawData { content: None }
            }),
            step: Some(Step::One),
            skip_save: true,
            skip_load_jig: true,
            bg_tab: Some(MenuTabKind::Image),
            content_tab: Some(MenuTabKind::Video),
        }
    }
}

pub fn init(jig_id: JigId, _module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS
            .set(DebugSettings::debug(Some(InitData {
                stickers: vec![
                    // InitSticker::Text, InitSticker::Sprite
                ],
            })))
            .unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
