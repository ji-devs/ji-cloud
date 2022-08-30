use super::super::state::Sidebar;
use crate::base::state::Base;
use components::{
    image::{
        search::{
            callbacks::Callbacks as ImageSearchCallbacks,
            state::{ImageSearchKind, ImageSearchOptions, State as ImageSearchState},
        },
        tag::ImageTag,
    },
    stickers::state::Stickers,
    tabs::MenuTabKind,
};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::module::body::{drag_drop::Mode, BodyExt};
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
                let mode = base.history.get_current().mode();

                let tags_priority = mode.map(|mode| get_image_tag_priorities_from_mode(mode));

                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Sticker,
                    tags_priority,
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

fn get_image_tag_priorities_from_mode(mode: Mode) -> Vec<ImageTag> {
    match mode {
        Mode::SettingTable => vec![ImageTag::Table],
        Mode::Sorting => vec![ImageTag::DragAndDrop],
        Mode::WordBuilder => vec![ImageTag::Boards, ImageTag::Book],
        Mode::SentenceBuilder => vec![ImageTag::Boards, ImageTag::Book],
        Mode::Matching => vec![ImageTag::DragAndDrop],
        Mode::DressUp => vec![ImageTag::Wardrobe],
        Mode::SceneBuilder => vec![],
    }
}
