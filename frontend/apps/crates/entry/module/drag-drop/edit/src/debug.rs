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
                        Next,
                        Step,
                    },
                    _groups::design::{Backgrounds, Sprite, Sticker, Text, Trace, TraceShape, BaseContent}
                }
            }
        }
    }
};
use components::stickers::{sprite::ext::*, text::ext::*};
use crate::base::sidebar::step_1::state::TabKind as Step1TabKind;
use crate::base::sidebar::step_2::state::TabKind as Step2TabKind;
use crate::base::sidebar::step_5::state::TabKind as Step5TabKind;
use components::traces::edit::state::DebugOptions as TracesOptions;
pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();

const IMAGE_UUID:&'static str = "f2e63cf2-ee11-11eb-9b68-4bf1f063ab1c";

pub const DEBUG_TEXT:&'static str = "Debug Text"; 

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data:Option<RawData>,
    pub step:Option<Step>,
    pub skip_save: bool,
    pub skip_load_jig: bool,
    pub step_1_tab: Option<Step1TabKind>,
    pub step_2_tab: Option<Step2TabKind>,
    pub step_5_tab: Option<Step5TabKind>,
    pub trace_opts: Option<TracesOptions>,
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
            step: Some(Step::Three),
            skip_save: true,
            skip_load_jig: true,
            step_1_tab: Some(Step1TabKind::StickerImage),
            step_2_tab: Some(Step2TabKind::Select),
            step_5_tab: Some(Step5TabKind::Settings),
            trace_opts: Some(TracesOptions {
                start_in_phase_draw: false
            })
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
                            target_transform: None,
                        }
                    ),
                    (-0.3, -0.3)
                ),
                (
                    InitSticker::Sprite, 
                    ItemKind::Interactive(
                        Interactive {
                            audio: None,
                            target_transform: None,
                        }
                    ),
                    (-0.3, 0.1)
                ),
                //( InitSticker::Sprite, ItemKind::Static)
            ],
            traces: vec![
                InitTrace::Ellipse(0.3, 0.4, 0.2, 0.1)
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
