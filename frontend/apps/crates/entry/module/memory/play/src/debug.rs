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
    domain::{
        jig::{
            module::body::{
                Image,
                ThemeChoice,
                Background, Backgrounds,
                Instructions, 
                memory::{
                    Content, 
                    Mode, 
                    ModuleData as RawData,
                    Card as RawCard, 
                    CardPair as RawCardPair
                }
            },
            JigId, module::ModuleId
        },
        image::ImageId,
        audio::AudioId
    },
    media::MediaLibrary
};
use components::stickers::{sprite::ext::*, text::ext::*};
pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();

//const IMAGE_UUID:&'static str = "bf2fe548-7ffd-11eb-b3ab-579026da8b36";
const IMAGE_UUID:&'static str = "9da11e0a-c17b-11eb-b863-570eea18a3bd";



#[derive(Debug, Default)]
pub struct DebugSettings {
    pub data:Option<RawData>,
    pub skip_load_jig: bool,
    pub no_shuffle: bool,
    pub ending: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitData {
    pub with_pairs: bool
}


impl DebugSettings {
    pub fn debug(init_data: Option<InitData>) -> DebugSettings {
        DebugSettings {
            //debug always has to have some data
            //otherwise it will fail at load time
            data: Some(
                if let Some(init_data) = init_data {
                    let mode = Mode::WordsAndImages;

                    RawData{
                        content: Some(Content {
                            mode,
                            theme: ThemeChoice::Override(ThemeId::Chalkboard), 
                            instructions: Instructions::default(),
                            pairs: if init_data.with_pairs {
                                crate::config::get_debug_pairs(mode, 3)
                                    .into_iter()
                                    .map(|(word_1, word_2)| {
                                        match mode {
                                            Mode::WordsAndImages => {
                                                RawCardPair(RawCard::Text(word_1), RawCard::Image(Some(Image {
                                                    id: ImageId(Uuid::parse_str(IMAGE_UUID).unwrap_ji()),
                                                    lib: MediaLibrary::User
                                                })))
                                            }
                                            _ => RawCardPair(
                                                RawCard::Text(word_1),
                                                RawCard::Text(word_2),
                                            ),
                                        }
                                    })
                                    .collect()
                            } else {
                                Vec::new()
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
            no_shuffle: true,
            ending: true
        }
    }
}

pub fn init(jig_id: JigId, module_id: ModuleId) {
    if jig_id == JigId(Uuid::from_u128(0)) {
        SETTINGS.set(DebugSettings::debug(Some(InitData{
            with_pairs: true
        }))).unwrap_ji();
        //SETTINGS.set(DebugSettings::debug(None)).unwrap_ji();
    } else {
        SETTINGS.set(DebugSettings::default()).unwrap_ji();
    }
}

pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
