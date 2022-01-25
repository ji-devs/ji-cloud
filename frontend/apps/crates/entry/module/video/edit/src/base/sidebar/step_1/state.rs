use super::super::state::Sidebar;
use crate::base::state::Base;
use components::{
    backgrounds::actions::Layer,
    color_select::state::State as ColorPickerState,
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchKind, ImageSearchOptions, State as ImageSearchState},
    },
    tabs::MenuTabKind,
};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::Background;
use std::rc::Rc;

const STR_SELECT_BACKGROUND_COLOR: &str = "Select background color";

pub struct Step1 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
}

impl Step1 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let kind = match crate::debug::settings().bg_tab {
            Some(kind) => kind,
            None => MenuTabKind::BackgroundImage,
        };

        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        Rc::new(Self { sidebar, tab })
    }
}

#[derive(Clone)]
pub enum Tab {
    BackgroundImage(Rc<ImageSearchState>),
    FillColor(Rc<ColorPickerState>),
    Overlay(Rc<ImageSearchState>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind: MenuTabKind) -> Self {
        match kind {
            MenuTabKind::BackgroundImage => {
                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Background,
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    base.backgrounds.set_layer(Layer::One, Background::Image(image));
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::BackgroundImage(Rc::new(state))
            }
            MenuTabKind::FillColor => {
                let state = ColorPickerState::new(
                    base.theme_id.read_only(),
                    None,
                    Some(String::from(STR_SELECT_BACKGROUND_COLOR)),
                    Some(clone!(base => move |color| {
                        base.backgrounds.set_layer(Layer::One, Background::Color(color));
                    })),
                );
                Self::FillColor(Rc::new(state))
            }
            MenuTabKind::Overlay => {
                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Overlay,
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    base.backgrounds.set_layer(Layer::Two, Background::Image(image));
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::Overlay(Rc::new(state))
            }

            _ => unimplemented!("unsupported tab kind!"),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::BackgroundImage(_) => MenuTabKind::BackgroundImage,
            Self::FillColor(_) => MenuTabKind::FillColor,
            Self::Overlay(_) => MenuTabKind::Overlay,
        }
    }

    pub fn as_index(&self) -> usize {
        match self {
            Self::BackgroundImage(_) => 0,
            Self::FillColor(_) => 1,
            Self::Overlay(_) => 2,
        }
    }
}
