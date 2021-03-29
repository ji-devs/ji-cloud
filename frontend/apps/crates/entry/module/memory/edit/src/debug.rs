use crate::data::state::*;
use cfg_if::cfg_if;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};

use std::cell::RefCell;
use std::rc::Rc;
use crate::data::{raw, state::*};

pub struct DebugSettings {
    pub data:Option<raw::GameData>,
    pub step:Option<Step>,
    //just used for words and images, but whatever
    pub image_search: Option<()>,
}

impl DebugSettings {
    pub fn default() -> DebugSettings {
        DebugSettings {
            data: None, 
            step: None, 
            image_search: None,
        }
    }
    pub fn duplicate(with_data: bool) -> DebugSettings {
        DebugSettings {
            data: Some(
                if with_data {
                    raw::GameData::duplicate_debug(
                        crate::config::get_init_words_iter(),
                        //vec!["foo"].iter(),
                        crate::config::get_themes_cloned()[1].clone()
                    )
                } else {
                    raw::GameData::new_duplicate()
                }
            ),
            step: Some(Step::Four), 
            image_search: None,
        }
    }
    /*
    pub fn words_and_images() -> DebugSettings {
        DebugSettings {
            data: Some(raw::GameData::words_and_images_debug(
                crate::config::get_init_words_iter(),
                crate::config::get_themes_cloned()[0].clone()
            )),
            step: Some(Step::Four), 
            image_search: None,
        }
    }
    */
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn settings() -> DebugSettings {
            DebugSettings::duplicate(true)
            //DebugSettings::default()
        }
    } else {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    }
}

