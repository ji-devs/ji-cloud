use crate::base::state::Base;
use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;
use components::{
    backgrounds::actions::Layer,
    image::search::{
        state::{State as ImageSearchState, ImageSearchOptions},
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
            None => TabKind::BgImage
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Rc::new(Self {
            base,
            tab
        })
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    BgImage,
    BgColor,
    BgOverlay,
    StickerImage,
    StickerText,
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BgImage => "image",
            Self::BgColor => "color",
            Self::BgOverlay => "overlay",
            Self::StickerImage => "image",
            Self::StickerText => "text",
        }
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
    pub fn new(base: Rc<Base>, kind:TabKind) -> Self {
        match kind {
            TabKind::BgImage => {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true, 
                    filters: true, 
                    recent: true,
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        base.backgrounds.set_layer(Layer::One, Background::Image(image));
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::BgImage(Rc::new(state))
            },
            TabKind::BgColor => {
                let state = ColorPickerState::new(base.theme_id.clone(), None, Some(clone!(base => move |color| {
                    base.backgrounds.set_layer(Layer::One, Background::Color(color));
                })));
                Self::BgColor(Rc::new(state))
            },
            TabKind::BgOverlay => {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true, 
                    filters: true, 
                    recent: true,
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        base.backgrounds.set_layer(Layer::Two, Background::Image(image));
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::BgOverlay(Rc::new(state))
            },

            TabKind::StickerImage => {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true, 
                    filters: true, 
                    recent: true,
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        Stickers::add_sprite(base.stickers.clone(), image);
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::StickerImage(Rc::new(state))
            },

            TabKind::StickerText=> {
                Self::StickerText
            },
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::BgImage(_) => TabKind::BgImage,
            Self::BgColor(_) => TabKind::BgColor,
            Self::BgOverlay(_) => TabKind::BgOverlay,
            Self::StickerImage(_) => TabKind::StickerImage,
            Self::StickerText => TabKind::StickerText,
        }
    }
}
