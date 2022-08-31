use super::super::state::Sidebar;
use crate::base::state::Base;
use components::{
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
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;

pub struct Step2 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
}

impl Step2 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let tab = Mutable::new(Tab::new(sidebar.base.clone(), MenuTabKind::Text));
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
    StickerImage(Rc<ImageSearchState>),
    StickerText,
}

impl Tab {
    pub fn new(base: Rc<Base>, kind: MenuTabKind) -> Self {
        match kind {
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
