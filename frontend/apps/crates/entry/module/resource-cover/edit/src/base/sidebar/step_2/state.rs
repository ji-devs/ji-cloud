use super::super::state::Sidebar;
use crate::base::state::Base;
use components::{
    image::search::{ImageSearch, ImageSearchCallbacks, ImageSearchKind, ImageSearchOptions},
    stickers::state::Stickers,
    tabs::MenuTabKind,
};
use dominator::clone;
use futures_signals::signal::Mutable;
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;

pub struct Step2 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
}

impl Step2 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let kind = match crate::debug::settings().content_tab {
            Some(kind) => kind,
            None => MenuTabKind::Image,
        };

        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        Rc::new(Self { sidebar, tab })
    }
}

#[derive(Clone)]
pub enum Tab {
    Text, // uses top-level state since it must be toggled from main too
    Image(Rc<ImageSearch>),
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

                let callbacks = ImageSearchCallbacks::new(Some(
                    clone!(base => move |image: Option<_>| {
                        let image = image.expect_ji("ImageSearchKind::Sticker should never call on_select with `None`");
                        Stickers::add_sprite(base.stickers.clone(), image);
                    }),
                ));
                let state = ImageSearch::new(opts, callbacks);

                Self::Image(state)
            }
            kind => unimplemented!("unsupported tab kind! {:?}", kind),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Text => MenuTabKind::Text,
            Self::Image(_) => MenuTabKind::Image,
        }
    }
}
