use crate::data::state::*;
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
use crate::{
    data::{raw, state::*},
    steps::sidebar::step_2::state::Tab as BgTab,
    steps::sidebar::step_3::state::Tab as ContentTab,
};
use once_cell::sync::OnceCell;
use utils::prelude::*;
use uuid::Uuid;
use shared::{
    domain::{
        jig::{
            module::body::{Sprite, Instructions, Renderable, Text},
            JigId, module::ModuleId
        },
        image::ImageId,
        audio::AudioId
    },
    media::MediaLibrary
};
use components::renderables::{sprite::*, text::*};

pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();
const STRING_UUID:&'static str = "bf2fe548-7ffd-11eb-b3ab-579026da8b36";

const DEBUG_TEXT:&'static str = "[{\"children\":[{\"text\":\"text from rust\",\"font\":\"\\\"Shesek - Regular\\\", \\\"Architects Daughter - Regular\\\"\",\"fontSize\":14,\"color\":\"#AFCBF4FF\"}],\"element\":\"P1\"}]";

#[derive(Debug)]
pub struct DebugSettings {
    pub data:Option<raw::ModuleData>,
    pub step:Option<Step>,
    pub selected_index: Option<usize>,
    pub live_save: bool,
    pub bg_tab: Option<BgTab>,
    pub content_tab: Option<ContentTab>,
    pub text_select_nonedit: bool,
    pub text_mock_box: bool 
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InitData {
    Text,
    Sticker,
}

impl DebugSettings {
    pub fn default() -> DebugSettings {
        DebugSettings {
            data: None, 
            step: None, 
            selected_index: None,
            live_save: true,
            bg_tab: None,
            content_tab: None,
            text_select_nonedit: false,
            text_mock_box: false

        }
    }
    pub fn debug(init_data: Option<Vec<InitData>>, selected_index: Option<usize>) -> DebugSettings {
        DebugSettings {
            data: Some(
                if let Some(init_data) = init_data.as_ref() {
                    raw::ModuleData{
                        theme_id: ThemeId::Chalkboard, 
                        instructions: Instructions::default(),
                        renderables: init_data.iter().map(|init| {
                            match init {
                                InitData::Text => Renderable::Text(Text::new(DEBUG_TEXT.to_string())),
                                InitData::Sticker => Renderable::Sprite(Sprite::new(ImageId(Uuid::parse_str(STRING_UUID).unwrap_ji()), MediaLibrary::Global))
                            }
                        }).collect(),
                        ..raw::ModuleData::default()
                    }
                } else {
                    raw::ModuleData{
                        theme_id: ThemeId::Chalkboard, 
                        instructions: Instructions::default(),
                        ..raw::ModuleData::default()
                    }
                }
            ),
            selected_index,
            step: Some(Step::One), 
            live_save: false,
            bg_tab: Some(BgTab::Color),
            content_tab: Some(ContentTab::Audio),
            text_select_nonedit: false,
            text_mock_box: false, 
        }
    }
}

pub fn init(jig_id: JigId, module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        //SETTINGS.set(DebugSettings::debug(Some(vec![InitData::Text]), None)).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(Some(vec![InitData::Text]), Some(0))).unwrap_ji();
        SETTINGS.set(DebugSettings::debug(Some(vec![InitData::Text, InitData::Sticker]), None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
