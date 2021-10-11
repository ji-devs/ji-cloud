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
use super::super::state::Sidebar;

pub struct Step1 {
    pub sidebar: Rc<Sidebar>,
    pub theme_selector: Rc<ThemeSelector>
}


impl Step1 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let callbacks = ThemeSelectorCallbacks::new(
            clone!(sidebar => move |theme_choice| {
                sidebar.base.set_theme(theme_choice);
            })
        );
        let theme_selector = Rc::new(ThemeSelector::new(sidebar.base.jig_id, sidebar.base.jig_theme_id.clone(), sidebar.base.theme_id.clone(), callbacks));

        Rc::new(Self {
            sidebar,
            theme_selector,
        })
    }
}

