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

use super::super::state::Sidebar;

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

        Rc::new(Self { sidebar, tab })
    }
}

pub fn next_kind(kind: &MenuTabKind) -> Option<MenuTabKind> {
    match kind {
        MenuTabKind::Text => Some(MenuTabKind::Image),
        _ => None,
    }
}

#[derive(Clone)]
pub enum Tab {
    Text, // uses top-level state since it must be toggled from main too
    Image(Rc<ImageSearchState>),
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
                    Stickers::add_sprite(base.stickers.clone(), image);
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            }

            _ => unimplemented!("unsupported tab kind!"),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Text => MenuTabKind::Text,
            Self::Image(_) => MenuTabKind::Image,
        }
    }
}
