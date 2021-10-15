use super::super::state::Sidebar;
use crate::base::state::Base;
use components::{
    backgrounds::actions::Layer,
    color_select::state::State as ColorPickerState,
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchCheckboxKind, ImageSearchOptions, State as ImageSearchState},
    },
    stickers::state::Stickers,
    tabs::MenuTabKind,
};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::Background;
use std::rc::Rc;

const STR_SELECT_BACKGROUND_COLOR: &'static str = "Select background color";

pub struct Step1 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
}

impl Step1 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let kind = match crate::debug::settings().step_1_tab {
            Some(kind) => kind,
            None => MenuTabKind::BackgroundImageFull,
        };

        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        Rc::new(Self { sidebar, tab })
    }
}

#[derive(Clone)]
pub enum Tab {
    BgImage(Rc<ImageSearchState>),
    BgColor(Rc<ColorPickerState>),
    BgOverlay(Rc<ImageSearchState>),
    StickerImage(Rc<ImageSearchState>),
    StickerText,
}

impl Tab {
    pub fn new(base: Rc<Base>, kind: MenuTabKind) -> Self {
        match kind {
            MenuTabKind::BackgroundImageFull => {
                let opts = ImageSearchOptions {
                    checkbox_kind: Some(ImageSearchCheckboxKind::BackgroundLayer1Filter),
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    base.backgrounds.set_layer(Layer::One, Background::Image(image));
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::BgImage(Rc::new(state))
            }
            MenuTabKind::BackgroundColor => {
                let state = ColorPickerState::new(
                    base.theme_id.clone(),
                    None,
                    Some(String::from(STR_SELECT_BACKGROUND_COLOR)),
                    Some(clone!(base => move |color| {
                        base.backgrounds.set_layer(Layer::One, Background::Color(color));
                    })),
                );
                Self::BgColor(Rc::new(state))
            }
            MenuTabKind::Overlay => {
                let opts = ImageSearchOptions {
                    checkbox_kind: Some(ImageSearchCheckboxKind::BackgroundLayer2Filter),
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    base.backgrounds.set_layer(Layer::Two, Background::Image(image));
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::BgOverlay(Rc::new(state))
            }

            MenuTabKind::Image => {
                let opts = ImageSearchOptions {
                    checkbox_kind: Some(ImageSearchCheckboxKind::StickersFilter),
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
            Self::BgImage(_) => MenuTabKind::BackgroundImageFull,
            Self::BgColor(_) => MenuTabKind::BackgroundColor,
            Self::BgOverlay(_) => MenuTabKind::Overlay,
            Self::StickerImage(_) => MenuTabKind::Image,
            Self::StickerText => MenuTabKind::Text,
        }
    }

    pub fn as_index(&self) -> usize {
        match self {
            Self::BgImage(_) => 0,
            Self::BgColor(_) => 1,
            Self::BgOverlay(_) => 2,
            Self::StickerImage(_) => 3,
            Self::StickerText => 4,
        }
    }
}
