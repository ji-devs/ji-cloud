use crate::data::*;
use cfg_if::cfg_if;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};

use std::cell::RefCell;
use std::rc::Rc;
use crate::data::*; 
use utils::components::image::{
    search::ImageSearchWidgetDebug, 
    data::*
};

pub struct DebugSettings {
    pub data:Option<raw::GameData>,
    pub step:Option<Step>,
    //just used for words and images, but whatever
    pub content_mode: ContentMode,
    pub image_search: Option<ImageSearchWidgetDebug>,
}

impl DebugSettings {
    pub fn default() -> DebugSettings {
        DebugSettings {
            data: None, 
            step: None, 
            content_mode: ContentMode::TextInit,
            image_search: None,
        }
    }
    pub fn duplicate() -> DebugSettings {
        DebugSettings {
            data: Some(raw::GameData::duplicate_debug(
                crate::config::get_init_words_iter(),
                crate::config::get_themes_cloned()[0].id.clone()
            )),
            step: Some(Step::Two), 
            content_mode: ContentMode::TextInit,
            image_search: None,
        }
    }
    pub fn words_and_images() -> DebugSettings {
        DebugSettings {
            data: Some(raw::GameData::words_and_images_debug(
                crate::config::get_init_words_iter(),
                crate::config::get_themes_cloned()[0].id.clone()
            )),
            step: Some(Step::Four), 
            content_mode: ContentMode::TextInit,
            image_search: None,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn settings() -> DebugSettings {
            DebugSettings::words_and_images()
        }
    } else {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    }
}

