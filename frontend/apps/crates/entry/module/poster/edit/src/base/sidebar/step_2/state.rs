use crate::base::{sidebar::state::Sidebar, state::Base};
use components::{
    audio::input::{AudioInput, AudioInputCallbacks, AudioInputOptions},
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchKind, ImageSearchOptions, State as ImageSearchState},
    },
    module::_groups::design::edit::design_ext::DesignExt,
    stickers::state::Stickers,
    tabs::MenuTabKind,
};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::module::body::Audio;
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;

pub struct Step2 {
    pub sidebar: Rc<Sidebar>,
    pub tab: Mutable<Tab>,
}

impl Step2 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let kind = match crate::debug::settings().content_tab {
            Some(kind) => kind,
            None => MenuTabKind::Text,
        };

        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        Rc::new(Self { sidebar, tab })
    }

    pub fn next_kind(&self) -> Option<MenuTabKind> {
        match self.tab.get_cloned().kind() {
            MenuTabKind::Text => Some(MenuTabKind::Image),
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
                    tags_priority: base.get_image_tag_priorities(),
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(
                    clone!(base => move |image: Option<_>| {
                        let image = image.expect_ji("ImageSearchKind::Sticker should never call on_select with `None`");
                        Stickers::add_sprite(base.stickers.clone(), image);
                    }),
                ));
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            }
            MenuTabKind::Audio => {
                let audio = {
                    let base = Rc::clone(&base);

                    let opts = AudioInputOptions::new(Some(base.audio.signal_cloned()));

                    let callbacks = AudioInputCallbacks::new(
                        Some(clone!(base => move |audio: Audio| {
                            base.set_audio(Some(audio));
                        })),
                        Some(clone!(base => move || {
                            base.set_audio(None);
                        })),
                    );

                    AudioInput::new(opts, callbacks)
                };

                Self::Audio(audio)
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
