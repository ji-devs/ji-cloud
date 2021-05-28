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
use utils::prelude::*;
use uuid::Uuid;
use shared::{
    domain::{
        jig::{
            module::body::{
                Sprite, Instructions, Sticker, Text, Trace, Transform, TraceShape,
                tapping_board::{Content, Mode as RawMode, ModuleData as RawData}
            },
            JigId, module::ModuleId
        },
        image::ImageId,
        audio::AudioId
    },
    media::MediaLibrary
};
use components::stickers::{sprite::ext::*, text::ext::*};
use crate::state::Mode;
use crate::steps::state::Step;
use crate::steps::sidebar::step_1::state::TabKind as BgTabKind;
use crate::steps::sidebar::step_2::state::TabKind as ContentTabKind;
use crate::steps::sidebar::step_3::state::TabKind as InteractionTabKind;
use components::traces::edit::state::DebugOptions as TracesOptions;
pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();

const STRING_UUID:&'static str = "bf2fe548-7ffd-11eb-b3ab-579026da8b36";
const DEBUG_TEXT:&'static str = "[{\"children\":[{\"text\":\"text from rust\",\"font\":\"\\\"Shesek - Regular\\\", \\\"Architects Daughter - Regular\\\"\",\"fontSize\":14,\"color\":\"#AFCBF4FF\"}],\"element\":\"P1\"}]";

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data:Option<RawData>,
    pub step:Option<Step>,
    pub skip_save: bool,
    pub bg_tab: Option<BgTabKind>,
    pub content_tab: Option<ContentTabKind>,
    pub interaction_tab: Option<InteractionTabKind>,
    pub trace_opts: Option<TracesOptions>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitData {
    pub stickers: Vec<InitSticker>,
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
                            mode: RawMode::Poster,
                            theme_id: ThemeId::Chalkboard, 
                            instructions: Instructions::default(),
                            stickers: init_data.stickers.iter().map(|init| {
                                match init {
                                    InitSticker::Text => Sticker::Text(Text::new(DEBUG_TEXT.to_string())),
                                    InitSticker::Sprite => Sticker::Sprite(Sprite::new(ImageId(Uuid::parse_str(STRING_UUID).unwrap_ji()), MediaLibrary::Global))
                                }
                            }).collect(),
                            traces: init_data.traces.iter().map(|init| {
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
                            }).collect(),
                            //traces,
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
            bg_tab: Some(BgTabKind::Image),
            content_tab: Some(ContentTabKind::Image),
            interaction_tab: Some(InteractionTabKind::Text),
            trace_opts: Some(TracesOptions {
                start_in_phase_draw: false
            })
        }
    }
}

pub fn init(jig_id: JigId, module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS.set(DebugSettings::debug(Some(InitData{
            stickers: vec![InitSticker::Text, InitSticker::Sprite],
            traces: vec![
                //InitTrace::Ellipse(0.3, 0.4, 0.2, 0.1)
            ]
        }))).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(None, None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
