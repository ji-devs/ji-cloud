#![allow(dead_code)]

use components::stickers::{
    embed::types::{EmbedExt, ParseUrlExt},
    sprite::ext::*,
};
use once_cell::sync::OnceCell;
use shared::{
    domain::{
        asset::AssetId,
        image::ImageId,
        module::{
            body::{
                Image, ModuleAssist,
                _groups::design::{
                    Backgrounds, BaseContent, Embed, EmbedHost, Sprite, Sticker, Text,
                    YoutubeEmbed, YoutubeUrl,
                },
                video::{Content, Mode, ModuleData as RawData},
            },
            ModuleId,
        },
    },
    media::MediaLibrary,
};
use utils::prelude::*;
use uuid::Uuid;

pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();

const IMAGE_UUID: &str = "e84dd7fe-c92d-11eb-8c82-cfd1d3fd13ff";

pub const DEBUG_TEXT: &str = "Hello World this is a long line of text";

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
    Embed,
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
                            theme: ThemeId::Chalkboard,
                            instructions: ModuleAssist {
                                text: Some("Heya World!".to_string()),
                                ..ModuleAssist::default()
                            },
                            feedback: ModuleAssist::default(),
                            stickers: init_data
                                .stickers
                                .iter()
                                .map(|init| match init {
                                    InitSticker::Text => {
                                        let text = Text::from_str(DEBUG_TEXT);
                                        Sticker::Text(text)
                                    }
                                    InitSticker::Sprite => Sticker::Sprite(Sprite::new(Image {
                                        id: ImageId(Uuid::parse_str(IMAGE_UUID).unwrap_ji()),
                                        lib: MediaLibrary::Global,
                                    })),
                                    InitSticker::Embed => {
                                        let youtube_url =
                                            YoutubeUrl::try_parse("LUQksiZ2TVw".to_string())
                                                .unwrap_ji();

                                        let youtube = YoutubeEmbed::new(youtube_url)
                                            .captions(true)
                                            .autoplay(true);

                                        let host = EmbedHost::Youtube(youtube);

                                        Sticker::Embed(Embed::new(host))
                                    }
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

pub fn init(asset_id: AssetId, _module_id: ModuleId) {
    if asset_id.uuid() == &Uuid::from_u128(0) {
        SETTINGS
            .set(DebugSettings::debug(Some(InitData {
                stickers: vec![
                    InitSticker::Text,
                    InitSticker::Embed, //InitSticker::Sprite
                ],
            })))
            .unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    SETTINGS.get().unwrap_ji()
}
