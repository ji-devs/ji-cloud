#![allow(unused_imports)]
#![allow(dead_code)]

use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};

use components::stickers::sprite::ext::*;
use once_cell::sync::OnceCell;
use shared::{
    domain::{
        asset::AssetId,
        image::ImageId,
        {
            jig::JigId,
            module::body::{
                Image, ModuleAssist,
                _groups::cards::{
                    BaseContent, Card as RawCard, CardContent as RawCardContent,
                    CardPair as RawCardPair, Mode,
                },
                memory::{Content, ModuleData as RawData, PlayerSettings},
            },
            module::ModuleId,
        },
    },
    media::MediaLibrary,
};
use utils::prelude::*;
use uuid::Uuid;
pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();

//const IMAGE_UUID:&'static str = "bf2fe548-7ffd-11eb-b3ab-579026da8b36";
const IMAGE_UUID: &str = "9da11e0a-c17b-11eb-b863-570eea18a3bd";

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data: Option<RawData>,
    pub skip_load_jig: bool,
    pub no_shuffle: bool,
    pub ending: bool,
}

#[derive(Clone, Debug, PartialEq)]
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
                        player_settings: PlayerSettings::default(),
                        base: BaseContent {
                            mode,
                            theme: ThemeId::Chalkboard,
                            instructions: ModuleAssist::default(),
                            pairs: if init_data.with_pairs {
                                crate::config::get_debug_pairs(mode, 3)
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
            skip_load_jig: true,
            no_shuffle: true,
            ending: true,
        }
    }
}

pub fn init(asset_id: AssetId, _module_id: ModuleId) {
    if asset_id.uuid() == &Uuid::from_u128(0) {
        SETTINGS
            .set(DebugSettings::debug(Some(InitData { with_pairs: true })))
            .unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
