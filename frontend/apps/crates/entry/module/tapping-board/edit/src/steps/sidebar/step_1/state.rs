use crate::steps::state::{Step, Base};
use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;
use components::{
    image::search::state::{State as ImageSearchState, ImageSearchOptions},
    color_select::state::{State as ColorPickerState},
};
pub struct Step1 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step1 {
    pub fn new(base: Rc<Base>) -> Self {

        let kind = match crate::debug::settings().bg_tab {
            Some(kind) => kind,
            None => TabKind::Image
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Self {
            base,
            tab
        }
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
        let theme_id = base.theme_id.get_cloned();

        match kind {
            TabKind::Image => {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true, 
                    filters: true, 
                };

                let state = ImageSearchState::new(opts, Some(|id, lib| {
                    log::info!("Image selected: {:?} {:?}", id, lib);
                }));
                Self::Image(Rc::new(state))
            },
            TabKind::Color => {
                let state = ColorPickerState::new(Some(theme_id), None, Some(|color| {
                    log::info!("Color selected: {:?}", color);
                }));
                Self::Color(Rc::new(state))
            },
            TabKind::Overlay => {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true, 
                    filters: true, 
                };

                let state = ImageSearchState::new(opts, Some(|id, lib| {
                    log::info!("Image selected: {:?} {:?}", id, lib);
                }));
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
