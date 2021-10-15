use super::super::state::Sidebar;
use crate::base::state::Base;
use components::{
    backgrounds::actions::Layer,
    color_select::state::State as ColorPickerState,
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchCheckboxKind, ImageSearchOptions, State as ImageSearchState},
    },
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
        let kind = match crate::debug::settings().bg_tab {
            Some(kind) => kind,
            None => MenuTabKind::Image,
        };

        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        Rc::new(Self { sidebar, tab })
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
    pub fn new(base: Rc<Base>, kind: MenuTabKind) -> Self {
        match kind {
            MenuTabKind::Image => {
                let opts = ImageSearchOptions {
                    checkbox_kind: Some(ImageSearchCheckboxKind::BackgroundLayer1Filter),
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    base.backgrounds.set_layer(Layer::One, Background::Image(image));
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            }
            MenuTabKind::Color => {
                let state = ColorPickerState::new(
                    base.theme_id.clone(),
                    None,
                    Some(String::from(STR_SELECT_BACKGROUND_COLOR)),
                    Some(clone!(base => move |color| {
                        base.backgrounds.set_layer(Layer::One, Background::Color(color));
                    })),
                );
                Self::Color(Rc::new(state))
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

                Self::Overlay(Rc::new(state))
            }

            _ => unimplemented!("unsupported tab kind!"),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Image(_) => MenuTabKind::Image,
            Self::Color(_) => MenuTabKind::Color,
            Self::Overlay(_) => MenuTabKind::Overlay,
        }
    }

    pub fn as_index(&self) -> usize {
        match self {
            Self::Image(_) => 0,
            Self::Color(_) => 1,
            Self::Overlay(_) => 2,
        }
    }
}
