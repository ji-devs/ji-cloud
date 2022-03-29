use components::module::_groups::cards::edit::{config, debug::DebugSettings as BaseDebugSettings};
use components::tabs::MenuTabKind;
use once_cell::sync::OnceCell;
use shared::domain::jig::{
    module::{
        body::{
            Instructions,
            _groups::cards::{
                BaseContent, Card as RawCard, CardContent as RawCardContent,
                CardPair as RawCardPair, Mode, Step,
            },
            flashcards::{Content, ModuleData as RawData, PlayerSettings},
        },
        ModuleId,
    },
    JigId,
};
use utils::prelude::*;
use uuid::Uuid;
pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();

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
                let mode = Mode::Lettering;

                RawData {
                    content: Some(Content {
                        player_settings: PlayerSettings {
                            display_mode:
                                shared::domain::jig::module::body::flashcards::DisplayMode::Single,
                            swap: false,
                        },
                        base: BaseContent {
                            mode,
                            theme: ThemeId::Chalkboard,
                            instructions: Instructions::default(),
                            pairs: if init_data.with_pairs {
                                config::get_debug_pairs(mode)
                                    .into_iter()
                                    .map(|(word_1, word_2)| match mode {
                                        Mode::WordsAndImages => RawCardPair(
                                            RawCard {
                                                audio: None,
                                                card_content: RawCardContent::Text(word_1),
                                            },
                                            RawCard {
                                                audio: None,
                                                card_content: RawCardContent::Image(None),
                                            },
                                        ),
                                        Mode::Images => RawCardPair(
                                            RawCard {
                                                audio: None,
                                                card_content: RawCardContent::Image(None),
                                            },
                                            RawCard {
                                                audio: None,
                                                card_content: RawCardContent::Image(None),
                                            },
                                        ),
                                        _ => RawCardPair(
                                            RawCard {
                                                audio: None,
                                                card_content: RawCardContent::Text(word_1),
                                            },
                                            RawCard {
                                                audio: None,
                                                card_content: RawCardContent::Text(word_2),
                                            },
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
            step: Some(Step::Four),
            skip_save: true,
            skip_load_jig: true,
        }
    }
}

pub fn init(jig_id: JigId, _module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        //SETTINGS.set(DebugSettings::debug(Some(InitData { with_pairs: true }))).unwrap_ji();
        SETTINGS.set(DebugSettings::debug(None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
