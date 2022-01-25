use components::stickers::{sprite::ext::*, text::ext::*};
use components::tabs::MenuTabKind;
use once_cell::sync::OnceCell;
use shared::{
    domain::{
        image::ImageId,
        jig::{
            module::{
                body::{
                    Image, Instructions, Transform,
                    _groups::design::{
                        Backgrounds, Sprite, Sticker, Text, Trace, TraceKind, TraceShape,
                    },
                    drag_drop::{
                        Content, Hint, Interactive, Item, ItemKind, Mode, ModuleData as RawData,
                        PlaySettings, Step, TargetArea,
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
pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();

const IMAGE_UUID: &str = "f2e63cf2-ee11-11eb-9b68-4bf1f063ab1c";

pub const DEBUG_TEXT: &str = "Debug Text";

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data: Option<RawData>,
    pub step: Option<Step>,
    pub skip_save: bool,
    pub skip_load_jig: bool,
    pub draw_kind: Option<TraceKind>,
    pub step_1_tab: Option<MenuTabKind>,
    pub step_2_tab: Option<MenuTabKind>,
    pub step_5_tab: Option<MenuTabKind>,
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
                RawData{
                        content: Some(Content {
                            mode: Mode::SettingTable,
                            target_areas: init_data.traces.iter().map(|init| {
                                let trace = {
                                    match init {
                                        InitTrace::Ellipse(x, y, w, h) => {
                                            let mut transform = Transform::identity();
                                            transform.set_translation_2d(*x, *y);
                                            Trace {
                                                shape: TraceShape::Ellipse(*w, *h),
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
                                            text.transform.rotate_z(1.5);

                                            Sticker::Text(text)
                                        },
                                        InitSticker::Sprite => {
                                            let mut sprite = Sprite::new(Image {
                                                id: ImageId(Uuid::parse_str(IMAGE_UUID).unwrap_ji()),
                                                lib: MediaLibrary::Global
                                            });

                                            sprite.transform.set_scale_2d(0.3, 0.3);
                                            sprite.transform.set_translation_2d(*translation_x, *translation_y);
                                            sprite.transform.rotate_z(1.5);
                                            Sticker::Sprite(sprite)
                                        }
                                    }
                                };

                                Item {
                                    sticker,
                                    kind: item_kind.clone()
                                }
                            }).collect(),
                            theme: ThemeId::Chalkboard,
                            instructions: Instructions::default(),
                            backgrounds: Backgrounds {
                                layer_1: None, //Some(Background::Color(hex_to_rgba8("#ff0000"))),
                                layer_2: None,
                            },
                            play_settings: PlaySettings {
                                hint: Hint::None,
                                ..PlaySettings::default()
                            },
                            ..Content::default()
                        })
                    }
            } else {
                RawData { content: None }
            }),
            step: Some(Step::Two),
            draw_kind: None,
            skip_save: true,
            skip_load_jig: true,
            step_1_tab: Some(MenuTabKind::Image),
            step_2_tab: Some(MenuTabKind::Select),
            step_5_tab: Some(MenuTabKind::PlaySettings),
        }
    }
}

pub fn init(jig_id: JigId, _module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS
            .set(DebugSettings::debug(Some(InitData {
                stickers: vec![
                    (InitSticker::Text, ItemKind::Static, (0.3, 0.3)),
                    (
                        InitSticker::Text,
                        ItemKind::Interactive(Interactive {
                            audio: None,
                            target_transform: None,
                        }),
                        (-0.3, -0.3),
                    ),
                    (
                        InitSticker::Sprite,
                        ItemKind::Interactive(Interactive {
                            audio: None,
                            target_transform: None,
                        }),
                        (-0.3, 0.1),
                    ),
                    //( InitSticker::Sprite, ItemKind::Static)
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
