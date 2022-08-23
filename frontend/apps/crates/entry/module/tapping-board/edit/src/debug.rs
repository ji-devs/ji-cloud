#![allow(dead_code)]
use components::stickers::sprite::ext::*;
use components::tabs::MenuTabKind;
use once_cell::sync::OnceCell;
use shared::{
    domain::{
        asset::AssetId,
        audio::AudioId,
        image::ImageId,
        module::body::{
            Audio, Image, Instructions, Transform,
            _groups::design::{
                Backgrounds, BaseContent, Sprite, Sticker, Text, Trace, TraceKind, TraceShape,
            },
            tapping_board::{Content, Mode, ModuleData as RawData, Step},
        },
        module::ModuleId,
    },
    media::MediaLibrary,
};
use utils::prelude::*;
use uuid::Uuid;
pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();
//const IMAGE_UUID:&'static str = "bf2fe548-7ffd-11eb-b3ab-579026da8b36";
const IMAGE_UUID: &str = "9da11e0a-c17b-11eb-b863-570eea18a3bd";
const AUDIO_UUID: &str = "734314da-0b07-11ec-95f0-2b4855fa3cb8";

const DEBUG_TEXT: &str = "Text from rust";

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data: Option<RawData>,
    pub step: Option<Step>,
    pub skip_save: bool,
    pub skip_load_jig: bool,
    pub bg_tab: Option<MenuTabKind>,
    pub content_tab: Option<MenuTabKind>,
    pub interaction_tab: Option<MenuTabKind>,
    pub settings_tab: Option<MenuTabKind>,
    pub draw_kind: Option<TraceKind>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitData {
    pub stickers: Vec<InitSticker>,
    pub traces: Vec<InitTrace>,
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
                        mode: Mode::Words,
                        traces: init_data
                            .traces
                            .iter()
                            .map(|init| match init {
                                InitTrace::Ellipse(x, y, w, h) => {
                                    let mut transform = Transform::identity();
                                    transform.set_translation_2d(*x, *y);
                                    Trace {
                                        shape: TraceShape::Ellipse(*w, *h),
                                        transform,
                                        kind: TraceKind::Regular,
                                        audio: Some(Audio {
                                            id: AudioId(Uuid::parse_str(AUDIO_UUID).unwrap_ji()),
                                            lib: MediaLibrary::User,
                                        }),
                                        text: Some("hello world".to_string()),
                                    }
                                }
                            })
                            .collect(),
                        base: BaseContent {
                            theme: ThemeId::Chalkboard,
                            instructions: Instructions::default(),
                            feedback: Instructions::default(),
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
            step: Some(Step::Three),
            skip_save: true,
            skip_load_jig: true,
            bg_tab: Some(MenuTabKind::BackgroundImage),
            content_tab: Some(MenuTabKind::Text),
            interaction_tab: Some(MenuTabKind::Audio),
            settings_tab: Some(MenuTabKind::PlaySettings),
            draw_kind: None,
        }
    }
}

pub fn init(asset_id: AssetId, _module_id: ModuleId) {
    if asset_id.uuid() == &Uuid::from_u128(0) {
        SETTINGS
            .set(DebugSettings::debug(Some(InitData {
                stickers: vec![
                    InitSticker::Text, //InitSticker::Sprite
                ],
                traces: vec![InitTrace::Ellipse(0.3, 0.4, 0.2, 0.1)],
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
