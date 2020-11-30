use crate::data::*;
use cfg_if::cfg_if;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use shared::media::MediaLibraryKind;

use std::cell::RefCell;
use std::rc::Rc;
use crate::data::*; 
use utils::components::image_search::LibraryImage;

pub const DEBUG_STEP:usize = 1;
pub const DEBUG_THEME_INDEX:usize = 0;


pub struct DebugSettings {
    pub poster:Option<raw::Poster>,
    pub tool: Tool,
    pub image_search: Option<ImageSearch>,
}

pub struct ImageSearch {
    pub results: Option<Vec<LibraryImage>>,
    pub is_published: Option<Option<bool>>, 
}

impl ImageSearch {
    pub fn new() -> Self {
        Self {
            results: None,
            is_published: Some(None)
            /*
            search_results: Some(vec![
                LibraryImage::from_string(
                    "foo".to_string(),
                    "bar".to_string(),
                    MediaLibraryKind::Global
                )
            ]),
            */
        }
    }
}
impl DebugSettings {
    pub fn default() -> Self {
        Self {
            poster: None, 
            tool: Tool::Layout,
            image_search: None,
        }
    }
    pub fn images() -> Self {
        Self {
            poster: None, 
            tool: Tool::Images,
            image_search: Some(ImageSearch::new()),
        }
    }

}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn settings() -> DebugSettings {
            DebugSettings::images()
        }
    } else {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    }
}

