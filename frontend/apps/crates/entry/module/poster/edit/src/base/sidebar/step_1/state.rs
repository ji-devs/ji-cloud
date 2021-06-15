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
    theme::design_selector::{
        state::ThemeSelector
    }
};
use shared::domain::jig::module::body::{Background, Image};
pub struct Step1 {
    pub base: Rc<Base>,
    pub theme_selector: Rc<ThemeSelector>
}


impl Step1 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let theme_selector = Rc::new(ThemeSelector {});

        Rc::new(Self {
            base,
            theme_selector,
        })
    }
}

