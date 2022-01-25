use crate::{
    color_select::state::State as ColorPickerState,
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchKind, ImageSearchOptions, State as ImageSearchState},
    },
    module::_groups::cards::edit::state::*,
    tabs::MenuTabKind,
    theme_selector::state::{ThemeSelector, ThemeSelectorCallbacks},
};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::Background;
use std::rc::Rc;

const STR_SELECT_BACKGROUND_COLOR: &str = "Select background color";

pub struct Step2<RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
    pub tab: Mutable<Tab>,
    pub tab_index: Mutable<Option<usize>>,
}

impl<RawData: RawDataExt, E: ExtraExt> Step2<RawData, E> {
    pub fn new(base: Rc<CardsBase<RawData, E>>, tab_index: Mutable<Option<usize>>) -> Rc<Self> {
        let kind = match base.debug.step2_tab {
            Some(kind) => kind,
            None => MenuTabKind::Theme,
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Rc::new(Self {
            base,
            tab,
            tab_index,
        })
    }
}

#[derive(Clone)]
pub enum Tab {
    Theme(Rc<ThemeSelector>),
    BackgroundImage(Rc<ImageSearchState>),
    FillColor(Rc<ColorPickerState>),
}

impl Tab {
    pub fn new<RawData: RawDataExt, E: ExtraExt>(
        base: Rc<CardsBase<RawData, E>>,
        kind: MenuTabKind,
    ) -> Self {
        match kind {
            MenuTabKind::Theme => {
                let callbacks = ThemeSelectorCallbacks::new(clone!(base => move |theme| {
                    base.set_theme(theme);
                }));
                let state = ThemeSelector::new(
                    base.theme_id.read_only(),
                    callbacks,
                );
                Self::Theme(Rc::new(state))
            }
            MenuTabKind::BackgroundImage => {
                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Background,
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    base.set_bg(Background::Image(image));
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
                        base.set_bg(Background::Color(color));
                    })),
                );
                Self::FillColor(Rc::new(state))
            }

            _ => unimplemented!("unsupported tab kind!"),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Theme(_) => MenuTabKind::Theme,
            Self::BackgroundImage(_) => MenuTabKind::BackgroundImage,
            Self::FillColor(_) => MenuTabKind::FillColor,
        }
    }
    pub fn as_index(&self) -> usize {
        match self {
            Self::Theme(_) => 0,
            Self::BackgroundImage(_) => 1,
            Self::FillColor(_) => 2,
        }
    }
}
