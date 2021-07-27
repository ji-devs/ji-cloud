use crate::base::state::Base;
use std::rc::Rc;
use futures_signals::signal::Mutable;
use dominator::clone;
use components::{
    backgrounds::actions::Layer,
    image::search::{
        state::{State as ImageSearchState, ImageSearchOptions},
        callbacks::Callbacks as ImageSearchCallbacks
    },
    color_select::state::{State as ColorPickerState},
};
use shared::domain::jig::module::body::Background;
pub struct Step1 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step1 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {

        let kind = match crate::debug::settings().bg_tab {
            Some(kind) => kind,
            None => TabKind::Image
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
    Image,
    Color,
    Overlay
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Color => "color",
            Self::Overlay => "overlay",
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    //Image(Rc<ImageSearchState>),
    Image(Rc<ImageSearchState>),
    Color(Rc<ColorPickerState>),
    Overlay(Rc<ImageSearchState>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind:TabKind) -> Self {
        match kind {
            TabKind::Image => {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true, 
                    filters: true, 
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        base.backgrounds.set_layer(Layer::One, Background::Image(image));
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            },
            TabKind::Color => {
                let state = ColorPickerState::new(base.theme_id.clone(), None, Some(clone!(base => move |color| {
                    base.backgrounds.set_layer(Layer::One, Background::Color(color));
                })));
                Self::Color(Rc::new(state))
            },
            TabKind::Overlay => {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true, 
                    filters: true, 
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        base.backgrounds.set_layer(Layer::Two, Background::Image(image));
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::Overlay(Rc::new(state))
            }
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Image(_) => TabKind::Image,
            Self::Color(_) => TabKind::Color,
            Self::Overlay(_) => TabKind::Overlay,
        }
    }
}
