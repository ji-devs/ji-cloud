use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::module::body::Background;
use std::rc::Rc;

use crate::{
    color_select::ColorSelector,
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchKind, ImageSearchOptions, State as ImageSearchState},
    },
    module::_groups::cards::edit::state::{CardsBase, ExtraExt, RawDataExt},
    tabs::MenuTabKind,
};

const STR_FILL_COLOR: &str = "Fill color";

pub struct CustomBackground<RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
    pub on_close: Box<dyn Fn()>,
    pub colors_open: Mutable<bool>,
    pub color_state: Rc<ColorSelector>,
    pub background_state: Rc<ImageSearchState>,
    pub tab_kind: Mutable<Option<MenuTabKind>>,
}

impl<RawData: RawDataExt, E: ExtraExt> CustomBackground<RawData, E> {
    pub fn new(
        base: Rc<CardsBase<RawData, E>>,
        tab_kind: Mutable<Option<MenuTabKind>>,
        on_close: Box<dyn Fn()>,
    ) -> Rc<Self> {
        let color_state = ColorSelector::new(
            base.theme_id.read_only(),
            None,
            Some(String::from(STR_FILL_COLOR)),
            Some(clone!(base => move |color| {
                base.set_bg(Some(Background::Color(color)));
            })),
        );

        let opts = ImageSearchOptions {
            kind: ImageSearchKind::Background,
            ..ImageSearchOptions::default()
        };

        let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image: Option<_>| {
            base.set_bg(image.map(|image| Background::Image(image)));
        })));
        let background_state = Rc::new(ImageSearchState::new(opts, callbacks));

        Rc::new(Self {
            base,
            on_close,
            colors_open: Mutable::new(false),
            color_state,
            background_state,
            tab_kind,
        })
    }
}
