use std::{rc::Rc, marker::PhantomData};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::{StepExt, Background};

use crate::{
    backgrounds::actions::Layer,
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchKind, ImageSearchOptions, State as ImageSearchState},
    },
    tabs::MenuTabKind,
    module::{_common::edit::entry::prelude::BaseExt, _groups::design::edit::design_ext::DesignExt},
    color_select::state::State as ColorPickerState,
};

const STR_FILL_COLOR: &str = "Fill color";

pub struct CustomBackground<Step, Base>
where
    Step: StepExt + 'static,
    Base: BaseExt<Step> + DesignExt + 'static,
{
    pub base: Rc<Base>,
    pub on_close: Box<dyn Fn()>,
    pub colors_open: Mutable<bool>,
    pub color_state: Rc<ColorPickerState>,
    pub tab: Mutable<Tab>,
    pub tab_index: Mutable<Option<usize>>,
    _step: PhantomData<Step>,
}


impl<Step, Base> CustomBackground<Step, Base> where
    Step: StepExt + 'static,
    Base: BaseExt<Step> + DesignExt + 'static,
{
    pub fn new(base: Rc<Base>, on_close: Box<dyn Fn()>) -> Rc<Self> {
        let color_state = Rc::new(ColorPickerState::new(
            base.get_theme().read_only(),
            None,
            Some(String::from(STR_FILL_COLOR)),
            Some(clone!(base => move |color| {
                base.get_backgrounds().set_layer(Layer::One, Background::Color(color));
            })),
        ));

        let tab = Mutable::new(Tab::new(base.clone(), MenuTabKind::BackgroundImage));

        Rc::new(Self {
            base,
            on_close,
            colors_open: Mutable::new(false),
            color_state,
            tab,
            tab_index: Mutable::new(None),
            _step: PhantomData,
        })
    }
}

#[derive(Clone)]
pub enum Tab {
    BackgroundImage(Rc<ImageSearchState>),
    Overlay(Rc<ImageSearchState>),
}

impl Tab {
    pub fn new<Step, Base>(base: Rc<Base>, kind: MenuTabKind) -> Self
    where
        Step: StepExt + 'static,
        Base: BaseExt<Step> + DesignExt + 'static
    {
        match kind {
            MenuTabKind::BackgroundImage => {
                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Background,
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    base.get_backgrounds().set_layer(Layer::One, Background::Image(image));
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::BackgroundImage(Rc::new(state))
            }
            MenuTabKind::Overlay => {
                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Overlay,
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    base.get_backgrounds().set_layer(Layer::Two, Background::Image(image));
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::Overlay(Rc::new(state))
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
