use super::super::state::Sidebar;
use crate::base::state::Base;
use components::{
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchKind, ImageSearchOptions, State as ImageSearchState},
    },
    module::_groups::design::edit::design_ext::DesignExt,
    stickers::{embed::types::PartialEmbedHost, state::Stickers},
    tabs::MenuTabKind,
};
use dominator::clone;
use futures_signals::signal::Mutable;
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;

pub struct Step2 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
    //
    pub host: Mutable<Option<PartialEmbedHost>>,
}

impl Step2 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let kind = match crate::debug::settings().content_tab {
            Some(kind) => kind,
            None => MenuTabKind::Video,
        };

        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        let host: Mutable<Option<PartialEmbedHost>> =
            Mutable::new(sidebar.base.get_embed_sticker().map(
                |host: Rc<components::stickers::embed::state::Embed>| {
                    host.host.lock_ref().partial()
                },
            ));

        Rc::new(Self { sidebar, tab, host })
    }

    pub fn next_kind(&self) -> Option<MenuTabKind> {
        match self.tab.get_cloned().kind() {
            MenuTabKind::Video => Some(MenuTabKind::Text),
            MenuTabKind::Text => Some(MenuTabKind::Image),
            _ => None,
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
    pub fn new(base: Rc<Base>, kind: MenuTabKind) -> Self {
        match kind {
            MenuTabKind::Video => Self::Video,
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

            _ => unimplemented!("unsupported tab kind!"),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Video => MenuTabKind::Video,
            Self::Text => MenuTabKind::Text,
            Self::Image(_) => MenuTabKind::Image,
        }
    }
}
