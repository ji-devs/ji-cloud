use crate::steps::state::{Step, Base};
use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;
use components::{
    image::search::{
        state::{State as ImageSearchState, ImageSearchOptions},
        callbacks::Callbacks as ImageSearchCallbacks
    },
    audio_input::{
        options::AudioInputOptions,
        state::State as AudioInputState,
        callbacks::Callbacks as AudioCallbacks,
    },
    stickers::state::Stickers,
};
use shared::domain::jig::module::body::{Image, Audio};

pub struct Step2 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step2 {
    pub fn new(base: Rc<Base>) -> Self {

        let kind = match crate::debug::settings().content_tab {
            Some(kind) => kind,
            None => TabKind::Text
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Self {
            base,
            tab
        }
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    Text,
    Image,
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Image => "image",
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    Text, // uses top-level state since it must be toggled from main too
    Image(Rc<ImageSearchState>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind:TabKind) -> Self {
        match kind {
            TabKind::Text=> {
                Self::Text
            },
            TabKind::Image=> {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true, 
                    filters: true, 
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        Stickers::add_sprite(base.stickers.clone(), image);
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            }
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Text => TabKind::Text,
            Self::Image(_) => TabKind::Image,
        }
    }
}

