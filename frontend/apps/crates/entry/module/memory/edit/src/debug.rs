#![allow(unused_imports)]
#![allow(dead_code)]

use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};

use components::module::_groups::cards::edit::{config, debug::DebugSettings as BaseDebugSettings};
use components::tabs::MenuTabKind;
use once_cell::sync::OnceCell;
use shared::domain::jig::{
    module::{
        body::{
            Instructions, ThemeChoice,
            _groups::cards::{BaseContent, Card as RawCard, CardPair as RawCardPair, Mode, Step},
            memory::{Content, ModuleData as RawData, PlayerSettings},
        },
        ModuleId,
    },
    JigId,
};
use utils::prelude::*;
use uuid::Uuid;

pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();

//const IMAGE_UUID:&'static str = "bf2fe548-7ffd-11eb-b3ab-579026da8b36";
const IMAGE_UUID: &'static str = "9da11e0a-c17b-11eb-b863-570eea18a3bd";

pub const DEBUG_TEXT:&'static str = "{\"version\":\"0.1.0\",\"content\":[{\"children\":[{\"text\":\"text from rust\",\"element\":\"P1\"}]}]}";

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data: Option<RawData>,
    pub step: Option<Step>,
    pub skip_load_jig: bool,
    pub skip_save: bool,
    pub base: Option<BaseDebugSettings>,
}

#[derive(Debug, Default)]
pub struct InitData {
    pub with_pairs: bool,
}
impl DebugSettings {
    pub fn debug(init_data: Option<InitData>) -> DebugSettings {
        DebugSettings {
            //debug always has to have some data
            //otherwise it will fail at load time
            data: Some(if let Some(init_data) = init_data {
                // let mode = Mode::Duplicate; // single list not tabs
                //let mode = Mode::WordsAndImages; // single list with tabs
                let mode = Mode::Synonyms; // double list not tabs

                RawData {
                    content: Some(Content {
                        player_settings: PlayerSettings::default(),
                        base: BaseContent {
                            mode,
                            theme: ThemeChoice::Override(ThemeId::Chalkboard),
                            instructions: Instructions::default(),
                            pairs: if init_data.with_pairs {
                                config::get_debug_pairs(mode)
                                    .into_iter()
                                    .take(2)
                                    .map(|(word_1, word_2)| match mode {
                                        Mode::WordsAndImages => {
                                            RawCardPair(RawCard::Text(word_1), RawCard::Image(None))
                                        }
                                        _ => RawCardPair(
                                            RawCard::Text(word_1),
                                            RawCard::Text(word_2),
                                        ),
                                    })
                                    .collect()
                            } else {
                                Vec::new()
                            },
                            ..BaseContent::default()
                        },
                    }),
                }
            } else {
                RawData { content: None }
            }),
            base: Some(BaseDebugSettings {
                step1_tab: Some(MenuTabKind::Text),
                step2_tab: Some(MenuTabKind::Theme),
                step3_tab: Some(MenuTabKind::PlaySettings),
            }),
            step: Some(Step::One),
            skip_save: true,
            skip_load_jig: true,
        }
    }
}

pub fn init(jig_id: JigId, _module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS
            .set(DebugSettings::debug(Some(InitData { with_pairs: false })))
            .unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
