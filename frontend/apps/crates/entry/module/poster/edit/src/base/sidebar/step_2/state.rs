use crate::base::{sidebar::state::Sidebar, state::Base};
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
                    log::info!("{:?}", image);
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

    pub fn as_index(&self) -> usize {
        match self {
            Self::Text => 0,
            Self::Image(_) => 1,
        }
    }
}
