use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};

use components::stickers::{sprite::ext::*, text::ext::*};
use once_cell::sync::OnceCell;
use shared::{
    domain::{
        audio::AudioId,
        image::ImageId,
        jig::{
            module::{
                body::{
                    Audio, Image, Instructions, ThemeChoice, Transform,
                    _groups::design::{
                        Backgrounds, Sprite, Sticker, Text, Trace, TraceKind, TraceShape,
                    },
                    drag_drop::{
                        Content, Hint, Interactive, Item, ItemKind, Mode, ModuleData as RawData,
                        Next, PlaySettings, TargetArea,
                    },
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

cfg_if::cfg_if! {
    if #[cfg(debug_assertions)] {
        pub const DEBUGGING_EVALUATION_RESULT:bool = true;
        pub const DEBUGGING_EVALUATION_RESULT_ONLY_MATCH:bool = false;
    } else {
        pub const DEBUGGING_EVALUATION_RESULT:bool = false;
        pub const DEBUGGING_EVALUATION_RESULT_ONLY_MATCH:bool = false;
    }
}

pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();

const IMAGE_UUID: &'static str = "f2e63cf2-ee11-11eb-9b68-4bf1f063ab1c";
//const IMAGE_UUID:&'static str = "e84dd7fe-c92d-11eb-8c82-cfd1d3fd13ff";
const AUDIO_UUID: &'static str = "734314da-0b07-11ec-95f0-2b4855fa3cb8";

pub const DEBUG_TEXT: &'static str = "Hello World this is a long line of text";

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data: Option<RawData>,
    pub skip_load_jig: bool,
    pub skip_play: bool,
}

#[derive(Clone, Debug)]
pub struct InitData {
    pub stickers: Vec<(InitSticker, ItemKind, (f64, f64))>, //last param is translation in the sticker's transform
    pub traces: Vec<InitTrace>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InitSticker {
    Text,
    Sprite,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InitTrace {
    //x, y, w, h
    Ellipse(f64, f64, f64, f64, bool),
    Path(f64, f64, Vec<(f64, f64)>, bool),
    Rect(f64, f64, f64, f64, bool),
}

impl DebugSettings {
    pub fn debug(init_data: Option<InitData>) -> DebugSettings {
        DebugSettings {
            //debug always has to have some data
            //otherwise it will fail at load time
            data: Some(if let Some(init_data) = init_data {
                RawData{
                        content: Some(Content {
                            mode: Mode::SettingTable,
                            target_areas: init_data.traces.iter().map(|init| {
                                let trace = {
                                    match init {
                                        InitTrace::Ellipse(x, y, w, h, transform_more) => {
                                            let mut transform = Transform::identity();
                                            transform.set_translation_2d(*x, *y);
                                            if *transform_more {
                                                transform.rotate_z(1.5);
                                                transform.set_scale_2d(0.2, 1.3);
                                            }
                                            Trace {
                                                shape: TraceShape::Ellipse(*w, *h),
                                                transform,
                                                kind: TraceKind::Regular,
                                                audio: None,
                                                text: None,
                                            }
                                        },
                                        InitTrace::Path(x, y, path, transform_more) => {
                                            let mut transform = Transform::identity();
                                            transform.set_translation_2d(*x, *y);
                                            if *transform_more {
                                                transform.rotate_z(1.5);
                                                transform.set_scale_2d(0.2, 1.3);
                                            }
                                            Trace {
                                                shape: TraceShape::Path(path.clone()),
                                                transform,
                                                kind: TraceKind::Regular,
                                                audio: None,
                                                text: None,
                                            }
                                        },
                                        InitTrace::Rect(x, y, width, height, transform_more) => {
                                            let mut transform = Transform::identity();
                                            transform.set_translation_2d(*x, *y);
                                            if *transform_more {
                                                transform.rotate_z(1.5);
                                                transform.set_scale_2d(0.2, 1.3);
                                            }
                                            Trace {
                                                shape: TraceShape::Rect(*width, *height),
                                                transform,
                                                kind: TraceKind::Regular,
                                                audio: None,
                                                text: None,
                                            }
                                        }
                                    }
                                };

                                TargetArea { trace }
                            }).collect(),
                            items: init_data.stickers.iter().map(|(sticker_kind, item_kind, (translation_x, translation_y))| {
                                let sticker = {
                                    match sticker_kind {
                                        InitSticker::Text => {
                                            let value = components::text_editor::state::State::text_to_value(DEBUG_TEXT);
                                            let mut text = Text::new(value);

                                            text.transform.set_translation_2d(*translation_x, *translation_y);
                                            text.transform.rotate_z(1.0);

                                            Sticker::Text(text)
                                        },
                                        InitSticker::Sprite => {
                                            let mut sprite = Sprite::new(Image {
                                                id: ImageId(Uuid::parse_str(IMAGE_UUID).unwrap_ji()), 
                                                lib: MediaLibrary::Global
                                            });

                                            sprite.transform.set_translation_2d(*translation_x, *translation_y);
                                            sprite.transform.rotate_z(1.5);
                                            sprite.transform.set_scale_2d(0.2, 1.3);

                                            Sticker::Sprite(sprite)
                                        }
                                    }
                                };

                                Item {
                                    sticker,
                                    kind: item_kind.clone() 
                                }
                            }).collect(),
                            theme: ThemeChoice::Override(ThemeId::Chalkboard), 
                            instructions: Instructions::default(),
                            //feedback: Instructions::default(),
                            feedback: Instructions {
                                text: Some("good job!".to_string()),
                                audio: Some(Audio { id: AudioId(Uuid::parse_str(AUDIO_UUID).unwrap_ji()), lib: MediaLibrary::User}),
                            },
                            backgrounds: Backgrounds {
                                layer_1: None, //Some(Background::Color(hex_to_rgba8("#ff0000"))),
                                layer_2: None,
                            },
                            play_settings: PlaySettings {
                                hint: Hint::Highlight,
                                next: Next::ClickContinue,
                                ..PlaySettings::default()
                            },
                            ..Content::default()
                        })
                    }
            } else {
                RawData { content: None }
            }),
            skip_load_jig: true,
            skip_play: true,
        }
    }
}

pub fn init(jig_id: JigId, _module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS
            .set(DebugSettings::debug(Some(InitData {
                stickers: vec![
                    (InitSticker::Text, ItemKind::Static, (0.3, 0.3)),
                    /*
                    (
                        InitSticker::Text,
                        ItemKind::Interactive(
                            Interactive {
                                audio: None,
                                target_transform: {
                                    let mut t = Transform::identity();
                                    Some(t)
                                }
                            }
                        ),
                        (-0.3, -0.3)
                    ),
                    */
                    (
                        InitSticker::Sprite,
                        ItemKind::Interactive(Interactive {
                            audio: Some(Audio {
                                id: AudioId(Uuid::parse_str(AUDIO_UUID).unwrap_ji()),
                                lib: MediaLibrary::User,
                            }),
                            target_transform: {
                                let t = Transform::identity();
                                Some(t)
                            },
                        }),
                        (-0.3, 0.3),
                    ),
                    //( InitSticker::Sprite, ItemKind::Static)
                ],
                traces: vec![
                    InitTrace::Rect(0.6, 0.1, 0.1, 0.2, true),
                    InitTrace::Ellipse(0.3, 0.4, 0.2, 0.1, true),
                    InitTrace::Path(
                        0.6,
                        0.1,
                        vec![(0.1, 0.2), (0.1, 0.4), (0.3, 0.5), (0.3, 0.6)],
                        true,
                    ),
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
