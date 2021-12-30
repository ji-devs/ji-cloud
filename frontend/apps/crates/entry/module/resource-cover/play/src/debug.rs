#![allow(dead_code)]
use components::stickers::{sprite::ext::*, text::ext::*};
use once_cell::sync::OnceCell;
use shared::{
    domain::{
        image::ImageId,
        jig::{
            module::{
                body::{
                    Image, Instructions, ThemeChoice,
                    _groups::design::{Backgrounds, BaseContent, Sprite, Sticker, Text},
                    resource_cover::{Content, ModuleData as RawData},
                },
                ModuleId,
            },
            JigId,
        },
    },
    media::MediaLibrary,
};
use utils::prelude::*;
use uuid::Uuid;
pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();

const IMAGE_UUID: &str = "e84dd7fe-c92d-11eb-8c82-cfd1d3fd13ff";

pub const DEBUG_TEXT: &str = "Hello World!";

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data: Option<RawData>,
    pub skip_load_jig: bool,
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

impl DebugSettings {
    pub fn debug(init_data: Option<InitData>) -> DebugSettings {
        DebugSettings {
            //debug always has to have some data
            //otherwise it will fail at load time
            data: Some(if let Some(init_data) = init_data {
                RawData {
                    content: Some(Content {
                        base: BaseContent {
                            theme: ThemeChoice::Override(ThemeId::Chalkboard),
                            instructions: Instructions {
                                text: Some("Heya World!".to_string()),
                                ..Instructions::default()
                            },
                            stickers: init_data
                                .stickers
                                .iter()
                                .map(|init| match init {
                                    InitSticker::Text => {
                                        let value =
                                            components::text_editor::state::State::text_to_value(
                                                DEBUG_TEXT,
                                            );
                                        let text = Text::new(value);
                                        Sticker::Text(text)
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
            skip_load_jig: true,
        }
    }
}

pub fn init(jig_id: JigId, _module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS
            .set(DebugSettings::debug(Some(InitData {
                stickers: vec![
                    InitSticker::Text, //InitSticker::Sprite
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
