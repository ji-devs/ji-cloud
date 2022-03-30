use super::super::state::Sidebar;
use crate::base::state::Base;
use components::{
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchKind, ImageSearchOptions, State as ImageSearchState},
    },
    stickers::state::Stickers,
    tabs::MenuTabKind,
};
use dominator::clone;
use futures_signals::signal::Mutable;
use std::rc::Rc;

pub struct Step2 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
}

impl Step2 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let tab = Mutable::new(Tab::new(sidebar.base.clone(), MenuTabKind::Text));
        Rc::new(Self { sidebar, tab })
    }
}

#[derive(Clone)]
pub enum Tab {
    StickerImage(Rc<ImageSearchState>),
    StickerText,
}

impl Tab {
    pub fn new(base: Rc<Base>, kind: MenuTabKind) -> Self {
        match kind {
            MenuTabKind::Image => {
                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Sticker,
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    Stickers::add_sprite(base.stickers.clone(), image);
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::StickerImage(Rc::new(state))
            }
            MenuTabKind::Text => Self::StickerText,
            _ => unimplemented!("unsupported tab kind!"),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::StickerImage(_) => MenuTabKind::Image,
            Self::StickerText => MenuTabKind::Text,
        }
    }
}
