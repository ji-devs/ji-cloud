use super::super::state::Sidebar;
use crate::base::state::Base;
use components::{
    audio::input::{AudioInput, AudioInputCallbacks, AudioInputOptions},
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchKind, ImageSearchOptions, State as ImageSearchState},
    },
    stickers::state::Stickers,
    tabs::MenuTabKind,
};
use dominator::clone;
use futures_signals::signal::{Mutable, SignalExt};
use shared::domain::jig::module::body::Audio;
use std::rc::Rc;

pub struct Step2 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
}

impl Step2 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let kind = match crate::debug::settings().content_tab {
            Some(kind) => kind,
            None => MenuTabKind::Text,
        };

        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        Rc::new(Self { tab, sidebar })
    }

    pub fn next_kind(&self) -> Option<MenuTabKind> {
        match self.tab.get_cloned().kind() {
            MenuTabKind::Text => Some(MenuTabKind::Image),
            MenuTabKind::Image => Some(MenuTabKind::Audio),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    Text, // uses top-level state since it must be toggled from main too
    Image(Rc<ImageSearchState>),
    Audio(Rc<AudioInput>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind: MenuTabKind) -> Self {
        match kind {
            MenuTabKind::Text => Self::Text,
            MenuTabKind::Image => {
                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Sticker,
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    log::info!("{:?}", image);
                    Stickers::add_sprite(base.stickers.clone(), image);
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            }
            MenuTabKind::Audio => {
                let opts = AudioInputOptions::new(Some(
                    base.instructions
                        .signal_cloned()
                        .map(|instructions| instructions.audio),
                ));

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
            }
            _ => unimplemented!("unsupported tab kind!"),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Text => MenuTabKind::Text,
            Self::Image(_) => MenuTabKind::Image,
            Self::Audio(_) => MenuTabKind::Audio,
        }
    }
}
