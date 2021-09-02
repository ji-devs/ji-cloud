
use crate::base::state::Base;
use std::rc::Rc;
use futures_signals::signal::{Signal, Mutable, SignalExt};
use dominator::clone;
use components::{
    image::search::{
        state::{State as ImageSearchState, ImageSearchOptions},
        callbacks::Callbacks as ImageSearchCallbacks
    },
    audio::input::{
        AudioInputOptions,
        AudioInput,
        AudioInputCallbacks,
    },
    stickers::state::Stickers,
};
use shared::domain::jig::module::body::{Image, Audio};

pub struct Step3 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step3 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {

        let kind = match crate::debug::settings().content_tab {
            Some(kind) => kind,
            None => TabKind::Text
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Rc::new(Self {
            base,
            tab
        })
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    Text,
    Image,
    Audio,
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Image => "image",
            Self::Audio => "audio",
        }
    }
}

use std::pin::Pin;


#[derive(Clone)]
pub enum Tab {
    Text, // uses top-level state since it must be toggled from main too
    Image(Rc<ImageSearchState>),
    Audio(Rc<AudioInput>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind:TabKind) -> Self {
        match kind {
            TabKind::Text=> {
                Self::Text
            },
            TabKind::Image=> {
                let opts = ImageSearchOptions {
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        log::info!("{:?}", image);
                        Stickers::add_sprite(base.stickers.clone(), image);
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            }
            TabKind::Audio => {
                let opts = AudioInputOptions::new(
                    Some(base.instructions.signal_cloned().map(|instructions| instructions.audio))
                );

                let callbacks = AudioInputCallbacks::new(
                    Some(clone!(base => move |audio:Audio| {
                        base.set_instructions_audio(Some(audio));
                    })),
                    Some(clone!(base => move || {
                        base.set_instructions_audio(None);
                    })),
                );

                let state = AudioInput::new(opts, callbacks);

                Self::Audio(state)
            },
        }
    }


    pub fn kind(&self) -> TabKind {
        match self {
            Self::Text => TabKind::Text,
            Self::Image(_) => TabKind::Image,
            Self::Audio(_) => TabKind::Audio,
        }
    }
}

