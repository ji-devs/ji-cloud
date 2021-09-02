use crate::base::state::Base;
use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;
use components::{
    tabs::MenuTabKind,
    backgrounds::actions::Layer,
    image::search::{
        state::{State as ImageSearchState, ImageSearchOptions, ImageSearchCheckboxKind},
        callbacks::Callbacks as ImageSearchCallbacks
    },
    color_select::state::{State as ColorPickerState},
    stickers::state::Stickers,
};
use shared::domain::jig::module::body::{Background, Image};
pub struct Step1 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step1 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {

        let kind = match crate::debug::settings().step_1_tab {
            Some(kind) => kind,
            None => MenuTabKind::BackgroundImageFull
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Rc::new(Self {
            base,
            tab
        })
    }
}



#[derive(Clone)]
pub enum Tab {
    BgImage(Rc<ImageSearchState>),
    BgColor(Rc<ColorPickerState>),
    BgOverlay(Rc<ImageSearchState>),
    StickerImage(Rc<ImageSearchState>),
    StickerText
}

impl Tab {
    pub fn new(base: Rc<Base>, kind:MenuTabKind) -> Self {
        match kind {
            MenuTabKind::BackgroundImageFull => {
                let opts = ImageSearchOptions {
                    checkbox_kind: Some(ImageSearchCheckboxKind::BackgroundLayer1Filter),
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        base.backgrounds.set_layer(Layer::One, Background::Image(image));
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::BgImage(Rc::new(state))
            },
            MenuTabKind::BackgroundColor => {
                let state = ColorPickerState::new(base.theme_id.clone(), None, Some(clone!(base => move |color| {
                    base.backgrounds.set_layer(Layer::One, Background::Color(color));
                })));
                Self::BgColor(Rc::new(state))
            },
            MenuTabKind::Overlay => {
                let opts = ImageSearchOptions {
                    checkbox_kind: Some(ImageSearchCheckboxKind::BackgroundLayer2Filter),
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        base.backgrounds.set_layer(Layer::Two, Background::Image(image));
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::BgOverlay(Rc::new(state))
            },

            MenuTabKind::Image => {
                let opts = ImageSearchOptions {
                    checkbox_kind: Some(ImageSearchCheckboxKind::StickersFilter),
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        Stickers::add_sprite(base.stickers.clone(), image);
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::StickerImage(Rc::new(state))
            },

            MenuTabKind::Text=> {
                Self::StickerText
            },

            _ => unimplemented!("unsupported tab kind!")
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
}
