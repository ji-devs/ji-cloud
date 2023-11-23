use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::module::body::{Background, ModeExt, StepExt};
use std::{marker::PhantomData, rc::Rc};

use crate::{
    backgrounds::actions::Layer,
    color_select::ColorSelector,
    image::search::{ImageSearch, ImageSearchCallbacks, ImageSearchKind, ImageSearchOptions},
    module::{
        _common::edit::entry::prelude::BaseExt,
        _groups::design::edit::{design_ext::DesignExt, theme_background::ThemeBackground},
    },
    tabs::MenuTabKind,
};

const STR_FILL_COLOR: &str = "Fill color";

pub struct CustomBackground<Step, Mode, Base>
where
    Step: StepExt + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt<Step> + DesignExt<Mode> + 'static,
{
    pub base: Rc<Base>,
    pub on_close: Box<dyn Fn()>,
    pub colors_open: Mutable<bool>,
    pub color_state: Rc<ColorSelector>,
    pub tab: Mutable<Tab>,
    pub tab_kind: Mutable<Option<MenuTabKind>>,
    _step: PhantomData<Step>,
    _mode: PhantomData<Mode>,
}

impl<Step, Mode, Base> CustomBackground<Step, Mode, Base>
where
    Step: StepExt + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt<Step> + DesignExt<Mode> + 'static,
{
    pub fn new(state: Rc<ThemeBackground<Step, Mode, Base>>, on_close: Box<dyn Fn()>) -> Rc<Self> {
        let color_state = ColorSelector::new(
            state.base.get_theme().read_only(),
            None,
            Some(String::from(STR_FILL_COLOR)),
            Some(clone!(state => move |color| {
                state.base.get_backgrounds().set_layer(Layer::One, Some(Background::Color(color)));
            })),
        );

        let tab = Mutable::new(Tab::new(state.base.clone(), MenuTabKind::BackgroundImage));

        Rc::new(Self {
            base: state.base.clone(),
            on_close,
            colors_open: Mutable::new(false),
            color_state,
            tab,
            tab_kind: state.tab_kind.clone(),
            _step: PhantomData,
            _mode: PhantomData,
        })
    }
}

#[derive(Clone)]
pub enum Tab {
    BackgroundImage(Rc<ImageSearch>),
    Overlay(Rc<ImageSearch>),
}

impl Tab {
    pub fn new<Step, Mode, Base>(base: Rc<Base>, kind: MenuTabKind) -> Self
    where
        Step: StepExt + 'static,
        Mode: ModeExt + 'static,
        Base: BaseExt<Step> + DesignExt<Mode> + 'static,
    {
        match kind {
            MenuTabKind::BackgroundImage => {
                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Background,
                    tags_priority: base.get_image_tag_priorities(),
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(
                    clone!(base => move |image: Option<_>| {
                        base.get_backgrounds().set_layer(Layer::One, image.map(|image| Background::Image(image)));
                    }),
                ));
                let state = ImageSearch::new(opts, callbacks);

                Self::BackgroundImage(state)
            }
            MenuTabKind::Overlay => {
                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Overlay,
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(
                    clone!(base => move |image: Option<_>| {
                            base.get_backgrounds().set_layer(Layer::Two, image.map(|image| Background::Image(image)));
                    }),
                ));
                let state = ImageSearch::new(opts, callbacks);

                Self::Overlay(state)
            }

            kind => unimplemented!("unsupported tab kind! {:?}", kind),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::BackgroundImage(_) => MenuTabKind::BackgroundImage,
            Self::Overlay(_) => MenuTabKind::Overlay,
        }
    }

    pub fn as_index(&self) -> usize {
        match self {
            Self::BackgroundImage(_) => 0,
            Self::Overlay(_) => 1,
        }
    }
}
