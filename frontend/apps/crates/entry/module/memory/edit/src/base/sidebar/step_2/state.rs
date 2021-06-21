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
    theme_selector::state::{ThemeSelector, ThemeSelectorCallbacks},
};
use shared::domain::jig::module::body::{Background, Image};
pub struct Step2 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step2 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let kind = match crate::debug::settings().step2_tab {
            Some(kind) => kind,
            None => TabKind::Theme
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));


        Rc::new(Self {
            base,
            tab, 
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    Theme,
    Image,
    Color,
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Theme => "theme",
            Self::Image => "background-image",
            Self::Color => "background-color",
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    Theme(Rc<ThemeSelector>),
    Image(Rc<ImageSearchState>),
    Color(Rc<ColorPickerState>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind:TabKind) -> Self {
        match kind {
            TabKind::Theme => {
                let callbacks = ThemeSelectorCallbacks::new(
                    clone!(base => move |theme| {
                        base.set_theme(theme);
                    })
                );
                let state = ThemeSelector::new(base.jig_id, base.jig_theme_id.clone(), base.theme_id.clone(), callbacks);
                Self::Theme(Rc::new(state))
            },
            TabKind::Image => {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true, 
                    filters: true, 
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        base.set_bg(Background::Image(image));
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            },

            TabKind::Color => {
                let state = ColorPickerState::new(base.theme_id.clone(), None, Some(clone!(base => move |color| {
                    base.set_bg(Background::Color(color));
                })));
                Self::Color(Rc::new(state))
            }
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Theme(_) => TabKind::Theme,
            Self::Image(_) => TabKind::Image,
            Self::Color(_) => TabKind::Color,
        }
    }
}
