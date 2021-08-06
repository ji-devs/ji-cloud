use crate::base::state::Base;
use components::{
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchOptions, State as ImageSearchState},
    },
    stickers::state::Stickers,
};
use dominator::clone;
use futures_signals::signal::Mutable;
use std::rc::Rc;

pub struct Step2 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}

impl Step2 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let kind = match crate::debug::settings().content_tab {
            Some(kind) => kind,
            None => TabKind::Video,
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Rc::new(Self { base, tab })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    Video,
    Text,
    Image,
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Video => "video",
            Self::Text => "text",
            Self::Image => "image",
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    Video,
    Text, // uses top-level state since it must be toggled from main too
    Image(Rc<ImageSearchState>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind: TabKind) -> Self {
        match kind {
            TabKind::Video => Self::Video,
            TabKind::Text => Self::Text,
            TabKind::Image => {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true,
                    filters: true,
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    log::info!("{:?}", image);
                    Stickers::add_sprite(base.stickers.clone(), image);
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            }
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Video => TabKind::Video,
            Self::Text => TabKind::Text,
            Self::Image(_) => TabKind::Image,
        }
    }
}
