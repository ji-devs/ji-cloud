
use crate::base::{sidebar::state::Sidebar, state::Base};
use std::rc::Rc;
use futures_signals::signal::{Signal, Mutable, SignalExt};
use dominator::clone;
use components::{audio::input::{
        AudioInputOptions,
        AudioInput,
        AudioInputCallbacks,
    }, image::search::{callbacks::Callbacks as ImageSearchCallbacks, state::{ImageSearchCheckboxKind, ImageSearchOptions, State as ImageSearchState}}, stickers::state::Stickers, tabs::MenuTabKind};
use shared::domain::jig::module::body::{Image, Audio};

pub struct Step3 {
    pub sidebar: Rc<Sidebar>,
    pub tab: Mutable<Tab>,
}


impl Step3 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {

        let kind = match crate::debug::settings().content_tab {
            Some(kind) => kind,
            None => MenuTabKind::Text
        };

        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        Rc::new(Self {
            sidebar,
            tab
        })
    }
}

#[derive(Clone)]
pub enum Tab {
    Text, // uses top-level state since it must be toggled from main too
    Image(Rc<ImageSearchState>),
    Audio(Rc<AudioInput>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind:MenuTabKind) -> Self {
        match kind {
            MenuTabKind::Text=> {
                Self::Text
            },
            MenuTabKind::Image=> {
                let opts = ImageSearchOptions {
                    checkbox_kind: Some(ImageSearchCheckboxKind::StickersFilter),
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
            MenuTabKind::Audio => {
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

            _ => unimplemented!("unsupported tab kind!")
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Text => MenuTabKind::Text,
            Self::Image(_) => MenuTabKind::Image,
            Self::Audio(_) => MenuTabKind::Audio,
        }
    }

    pub fn as_index(&self) -> usize {
        match self {
            Self::Text => 0, 
            Self::Image(_) => 1, 
            Self::Audio(_) => 2,
        }
    }
}

