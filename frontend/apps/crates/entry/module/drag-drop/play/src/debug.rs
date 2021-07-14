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
    media::MediaLibrary,
    domain::{
        audio::AudioId, 
        image::ImageId, 
        jig::{
            JigId, 
            module::{
                ModuleId, 
                body::{
                    Image,
                    ThemeChoice,
                    Background,
                    Instructions,
                    Transform,
                    drag_drop::{
                        Content, Mode, ModuleData as RawData,
                        Item,
                        TargetArea,
                        ItemKind,
                        Interactive,
                        PlaySettings,
                        Hint,
                        Next
                    },
                    _groups::design::{Backgrounds, Sprite, Sticker, Text, Trace, TraceShape, BaseContent}
                }
            }
        }
    }
};
use components::stickers::{sprite::ext::*, text::ext::*};
pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();

//const IMAGE_UUID:&'static str = "bf2fe548-7ffd-11eb-b3ab-579026da8b36";
const IMAGE_UUID:&'static str = "9da11e0a-c17b-11eb-b863-570eea18a3bd";


pub const DEBUG_TEXT:&'static str = "Hello World this is a long line of text";

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data:Option<RawData>,
    pub skip_load_jig: bool,
    pub skip_play: bool,
}

#[derive(Clone, Debug)]
pub struct InitData {
    pub stickers: Vec<(InitSticker, ItemKind, (f64, f64))>, //last param is translation in the sticker's transform
    pub traces: Vec<InitTrace>
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
                            mode: Mode::SettingTable,
                            target_areas: init_data.traces.iter().map(|init| {
                                let trace = {
                                    match init {
                                        InitTrace::Ellipse(x, y, w, h) => {
                                            let mut transform = Transform::identity();
                                            transform.set_translation_2d(*x, *y);
                                            Trace {
                                                shape: TraceShape::Ellipse(*w, *h),
                                                transform
                                            }
                                        }
                                    }
                                };

                                TargetArea { trace, id: Uuid::new_v4() }
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
                                        InitSticker::Sprite => Sticker::Sprite(Sprite::new(Image {
                                            id: ImageId(Uuid::parse_str(IMAGE_UUID).unwrap_ji()), 
                                            lib: MediaLibrary::Global
                                        }))
                                    }
                                };

                                Item {
                                    sticker,
                                    kind: item_kind.clone() 
                                }
                            }).collect(),
                            theme: ThemeChoice::Override(ThemeId::Chalkboard), 
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
                    RawData{
                        content: None                    
                    }
                }
            ),
            skip_load_jig: true,
            skip_play: true,
        }
    }
}

pub fn init(jig_id: JigId, module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS.set(DebugSettings::debug(Some(InitData{
            stickers: vec![
                (InitSticker::Text, ItemKind::Static, (0.3, 0.3)),
                (
                    InitSticker::Text, 
                    ItemKind::Interactive(
                        Interactive {
                            audio: None,
                            target_offset: (0.0, 0.0).into()
                        }
                    ),
                    (-0.3, -0.3)
                ),
                //( InitSticker::Sprite, ItemKind::Static)
            ],
            traces: vec![
                InitTrace::Ellipse(0.3, 0.4, 0.2, 0.1),
                InitTrace::Ellipse(0.6, 0.1, 0.1, 0.1),
            ]
        }))).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
