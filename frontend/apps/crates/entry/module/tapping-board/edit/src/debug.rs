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
                Sprite, Instructions, Sticker, Text,
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
    pub traces: Option<TracesOptions>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InitData {
    Text,
    Sprite,
}

impl DebugSettings {
    pub fn debug(init_data: Option<Vec<InitData>>, selected_index: Option<usize>) -> DebugSettings {
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
                            stickers: init_data.iter().map(|init| {
                                match init {
                                    InitData::Text => Sticker::Text(Text::new(DEBUG_TEXT.to_string())),
                                    InitData::Sprite => Sticker::Sprite(Sprite::new(ImageId(Uuid::parse_str(STRING_UUID).unwrap_ji()), MediaLibrary::Global))
                                }
                            }).collect(),
                            ..Content::default()
                        })
                    }
                } else {
                    RawData{
                        content: None                    
                    }
                }
            ),
            step: Some(Step::Two),
            skip_save: true,
            bg_tab: Some(BgTabKind::Color),
            content_tab: Some(ContentTabKind::Image),
            interaction_tab: Some(InteractionTabKind::Text),
            traces: Some(TracesOptions {
                start_in_phase_draw: true
            })
        }
    }
}

pub fn init(jig_id: JigId, module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS.set(DebugSettings::debug(Some(vec![InitData::Text, InitData::Sprite]), None)).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(None, None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
